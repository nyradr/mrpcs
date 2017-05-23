use std::sync::{Arc, Mutex};
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{Instant, Duration};
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::io::Error;

use server::{Status, RecvHandle, RecvMess, TServInstance};
use ::BUFFER_SIZE;

/// Instance of a UDP server
#[derive(Clone)]
pub struct UdpServInstance{
    /* Port used by instance */
    port: u16,
    /* Instance status */
    status: Arc<Mutex<Status>>,
    /* UDP socket */
    sock: Arc<UdpSocket>,
    /* timeout value */
    timeout: Arc<Mutex<Duration>>,
    /* Client RTT timeouts */
    timeouts: Arc<Mutex<HashMap<SocketAddr, Instant>>>
}

impl UdpServInstance{

    /// Create a new UdpServInstance
    /// * 'port` : Server port
    /// * `timeout` : Listed and request timeout
    /// * `v4` : Listen for ipv4 address instead of ipv6
    pub fn new(port: u16, timeout: Duration, v4: bool)->UdpServInstance{
        let addr: IpAddr;
        if v4{
            addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        }else{
            addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
        }



        let sock = UdpSocket::bind((addr, port)).unwrap();
        let usi = UdpServInstance{
            port: port,
            status: Arc::new(Mutex::new(Status::Starting)),
            sock: Arc::new(sock),
            timeout: Arc::new(Mutex::new(timeout)),
            timeouts: Arc::new(Mutex::new(HashMap::new()))
        };
        usi.set_timeout(timeout);
        usi
    }
}

impl TServInstance for UdpServInstance{
    /* Test if the server status is RUNNING */
    fn is_running(&self) -> bool{
        let ref st = *self.status.lock().unwrap();

        match st{
            &Status::Running => true,
            _ => false
        }
    }

    /* Get the current server status */
    fn get_status(&self)->Status{
        self.status.lock().unwrap().clone()
    }

    /* Set the socket timeout
        d : timeout duration
    */
    fn set_timeout(&self, d: Duration){
        let _ = self.sock.set_read_timeout(Some(d));
        let _ = self.sock.set_write_timeout(Some(d));

        let mut tm = self.timeout.lock().unwrap();
        *tm = d;
    }

    /* Get the socket timeout */
    fn get_timeout(&self) -> Duration{
        *self.timeout.lock().unwrap()
    }

    /* Run a server instance
        tx : mpsc sender where send received datas
    */
    fn run(&mut self, tx: Sender<RecvHandle>){
        {
            let mut st = self.status.lock().unwrap();
            *st = Status::Running;
        }

        while self.is_running(){
            // receive data
            let mut buff = [0u8; BUFFER_SIZE];
            match self.sock.recv_from(&mut buff){
                Ok((len, addr)) =>{
                    let buff = &mut buff[..len];

                    // send data to handler
                    let hndl = RecvHandle::Mess(RecvMess::new(self.port, addr, buff.to_vec()));
                    let _ = tx.send(hndl);

                    // remove client from timeout
                    let mut tm = self.timeouts.lock().unwrap();
                    tm.remove(&addr);
                }
                Err(_) => {}
            }

            // Clear timed out requests
            let mut tm = self.timeouts.lock().unwrap();
            let mut ntm: HashMap<SocketAddr, Instant> = HashMap::new();
            for (a, t) in tm.iter(){
                let timeout = self.timeout.lock().unwrap();
                if t.elapsed() >= *timeout{
                    let  hndl = RecvHandle::Timeout(a.clone());
                    let _ = tx.send(hndl);
                }else{
                    ntm.insert(a.clone(), t.clone());
                }
            };
            *tm = ntm;
        }

        {
            let mut st = self.status.lock().unwrap();
            *st = Status::Stoped;
        }
    }

    /* Stop the running server */
    fn stop(&mut self){
        let mut st = self.status.lock().unwrap();
        *st = Status::Stoping;
    }

    /* Try to send data through the socket of this server */
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> Result<usize, Error>{
        // send buffer
        let mut buff = [0u8; BUFFER_SIZE];
        for i in 0usize..data.len(){
            buff[i] = data[i];
        }
        let len = self.sock.send_to(&buff, addr)?;

        // put request timeout
        let mut tm = self.timeouts.lock().unwrap();
        tm.insert(addr, Instant::now());
        Ok(len)
    }
}
