use std::sync::{Arc, Mutex};
use std::net::{UdpSocket, SocketAddr};
use std::time::{Instant, Duration};
use std::sync::mpsc::Sender;
use std::collections::HashMap;

use server::{Status, RecvHandle, RecvMess, TServInstance};
use ::BUFFER_SIZE;

/* Instance of a UDP server
*/
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

    /* Create a new UdpServInstance */
    pub fn new(port: u16, timeout: Duration)->UdpServInstance{
        let sock = UdpSocket::bind(("localhost", port)).unwrap();
        let usi = UdpServInstance{
            port: port,
            status: Arc::new(Mutex::new(Status::STARTING)),
            sock: Arc::new(sock),
            timeout: Arc::new(Mutex::new(timeout)),
            timeouts: Arc::new(Mutex::new(HashMap::new()))
        };
        usi.set_timeout(timeout);
        return usi;
    }
}

impl TServInstance for UdpServInstance{
    /* Test if the server status is RUNNING */
    fn is_running(&self) -> bool{
        let ref st = *self.status.lock().unwrap();

        match st{
            &Status::RUNNING => {return true;}
            _ => {
                return false;
            }
        }
    }

    /* Get the current server status */
    fn get_status(&self)->Status{
        let st = self.status.lock().unwrap();
        return st.clone();
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
        let tm = self.timeout.lock().unwrap();
        return *tm;
    }

    /* Run a server instance
        tx : mpsc sender where send received datas
    */
    fn run(&mut self, tx: Sender<RecvHandle>){
        {
            let mut st = self.status.lock().unwrap();
            *st = Status::RUNNING;
        }

        while self.is_running(){
            // receive data
            let mut buff = [0u8; BUFFER_SIZE];
            match self.sock.recv_from(&mut buff){
                Ok((len, addr)) =>{
                    let buff = &mut buff[..len];

                    // send data to handler
                    let hndl = RecvHandle::MESS(RecvMess::new(self.port, addr, buff.to_vec()));
                    let _ = tx.send(hndl);
                }
                Err(_) => {}
            }

            // Clear timed out requests
            let mut tm = self.timeouts.lock().unwrap();
            let mut ntm: HashMap<SocketAddr, Instant> = HashMap::new();
            for (a, t) in tm.iter(){
                let timeout = self.timeout.lock().unwrap();
                if t.elapsed() >= *timeout{
                    let  hndl = RecvHandle::TIMEOUT(a.clone());
                    let _ = tx.send(hndl);
                }else{
                    ntm.insert(a.clone(), t.clone());
                }
            };
            *tm = ntm;
        }

        {
            let mut st = self.status.lock().unwrap();
            *st = Status::STOPED;
        }
    }

    /* Stop the running server */
    fn stop(&mut self){
        let mut st = self.status.lock().unwrap();
        *st = Status::STOPING;
    }

    /* Try to send data through the socket of this server */
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> bool{
        if data.len() < BUFFER_SIZE{
            // send buffer
            let mut buff = [0u8; BUFFER_SIZE];
            for i in 0usize..data.len(){
                buff[i] = data[i];
            }
            let _ = self.sock.send_to(&buff, addr);

            // put request timeout
            let mut tm = self.timeouts.lock().unwrap();
            tm.insert(addr, Instant::now());
            return true;
        }

        return false;
    }
}
