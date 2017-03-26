use std::sync::{Arc};
use std::net::{UdpSocket, SocketAddr};
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, Sender};

use server::{Status, RecvHandle, TServInstance};
use ::BUFFER_SIZE;

/* Instance of a UDP server
*/
#[derive(Clone)]
pub struct UdpServInstance{
    /* Port used by instance */
    port: u16,
    /* Instance status */
    status: Status,
    /* UDP socket */
    sock: Arc<UdpSocket>,
}

impl UdpServInstance{

    /* Create a new UdpServInstance */
    pub fn new(port: u16, timeout: Duration)->UdpServInstance{
        let mut sock = UdpSocket::bind(("localhost", port)).unwrap();
        let usi = UdpServInstance{
            port: port,
            status: Status::STARTING,
            sock: Arc::new(sock)
        };
        usi.set_timeout(timeout);
        return usi;
    }
}

impl TServInstance for UdpServInstance{
    /* Test if the server status is RUNNING */
    fn is_running(&self) -> bool{
        match &self.status{
            RUNNING => {return true;}
            _ => {return false;}
        }
    }

    /* Get the current server status */
    fn get_status(&self)->&Status{
        return &self.status;
    }

    /* Set the socket timeout
        d : timeout duration
    */
    fn set_timeout(&self, d: Duration){
        self.sock.set_read_timeout(Some(d));
        self.sock.set_write_timeout(Some(d));
    }

    /* Get the socket timeout */
    fn get_timeout(&self) -> Option<Duration>{
        return self.sock.read_timeout().unwrap();
    }

    /* Run a server instance
        tx : mpsc sender where send received datas
    */
    fn run(&mut self, tx: Sender<RecvHandle>){
        self.status = Status::RUNNING;

        while self.is_running(){
            // receive data
            let mut buff = [0u8; BUFFER_SIZE];
            match self.sock.recv_from(&mut buff){
                Ok((len, addr)) =>{
                    let buff = &mut buff[..len];
                    println!("Server recv: {:?}", buff);

                    // send data to handler
                    let hndl = RecvHandle::new(self.port, addr, buff.to_vec());
                    tx.send(hndl);
                }
                Err(_) => {}
            }
        }

        self.status = Status::STOPED;
    }

    /* Stop the running server */
    fn stop(&mut self){
        self.status = Status::STOPING;
    }

    /* Try to send data through the socket of this server */
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> bool{
        if data.len() < BUFFER_SIZE{
            let mut buff = [0u8; BUFFER_SIZE];
            for i in 0usize..data.len(){
                buff[i] = data[i];
            }
            self.sock.send_to(&buff, addr);
            return true;
        }

        return false;
    }
}
