use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;
use std::thread;

use server::{Status, RecvHandle, TServInstance};
use ::BUFFER_SIZE;

pub struct TcpServInstance {
    /* Port used by the server */
    port: u16,
    /* Server status */
    status: Arc<Mutex<Status>>,
    /* TCP socket */
    sock: Arc<TcpListener>,
    /* Clients IO timeout */
    timeout: Arc<Mutex<Duration>>,
    /* List of connected peers */
    peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>
}

impl TcpServInstance{

    /* Create a new TcpServInstance
        port: server port
        timeout: listener timeout
    */
    pub fn new(port: u16, timeout: Duration)-> TcpServInstance{
        let mut sock = TcpListener::bind(("localhost", port)).unwrap();
        sock.set_nonblocking(true);
        let tsi = TcpServInstance {
            port: port,
            status: Arc::new(Mutex::new(Status::STARTING)),
            sock: Arc::new(sock),
            timeout: Arc::new(Mutex::new(timeout)),
            peers: Arc::new(Mutex::new(HashMap::new()))
        };
        return tsi;
    }
}

fn run_tcpstream(stream: TcpStream){

}

impl TServInstance for TcpServInstance{
    /* Test if the server status is RUNNING */
    fn is_running(&self) -> bool{
        let ref st = *self.status.lock().unwrap();

        match st{
            &Status::RUNNING => {return true;}
            x => {
                return false;
            }
        }
    }

    /* Get the current server status */
    fn get_status(&self) -> Status{
        let st = self.status.lock().unwrap();
        return st.clone();
    }

    /* Set the socket timeout
        d : timeout duration
    */
    fn set_timeout(&self, d: Duration){
        let mut tm = self.timeout.lock().unwrap();
        *tm = d;
    }

    /* Get the socket timeout */
    fn get_timeout(&self) -> Option<Duration>{
        let tm = self.timeout.lock().unwrap();
        return Some(tm.clone());
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
            for stream in self.sock.incoming(){
                let stream = stream.unwrap();

                {   // setting socket timeout
                    let timeout = *self.timeout.lock().unwrap();
                    stream.set_read_timeout(Some(timeout.clone()));
                    stream.set_write_timeout(Some(timeout.clone()));
                }

                let nstream = stream.try_clone().unwrap();
                let addr = nstream.peer_addr().unwrap();
                let mut peers = self.peers.lock().unwrap();
                peers.insert(addr, nstream);

                thread::spawn(move ||{
                    run_tcpstream(stream);
                });
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

    /* Try to send data through the socket of this server
        addr : target address
        data : data to send (must be shorter than the buffer length)
        return true in case of success
    */
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> bool {
        return false; // TODO
    }
}
