use std::sync::{Arc, Mutex};
use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;
use std::sync::mpsc::Sender;

use server::{Status, RecvHandle, TServInstance};
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
}

impl UdpServInstance{

    /* Create a new UdpServInstance */
    pub fn new(port: u16, timeout: Duration)->UdpServInstance{
        let sock = UdpSocket::bind(("localhost", port)).unwrap();
        let usi = UdpServInstance{
            port: port,
            status: Arc::new(Mutex::new(Status::STARTING)),
            sock: Arc::new(sock)
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
    }

    /* Get the socket timeout */
    fn get_timeout(&self) -> Option<Duration>{
        return self.sock.read_timeout().unwrap();
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
                    let hndl = RecvHandle::new(self.port, addr, buff.to_vec());
                    let _ = tx.send(hndl);
                }
                Err(_) => {}
            }
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
            let mut buff = [0u8; BUFFER_SIZE];
            for i in 0usize..data.len(){
                buff[i] = data[i];
            }
            let _ = self.sock.send_to(&buff, addr);
            return true;
        }

        return false;
    }
}
