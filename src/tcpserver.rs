use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, Sender};

use server::{Status, RecvHandle, TServInstance};
use ::BUFFER_SIZE;

pub struct TcpServInstance {
    /* Port used by the server */
    port: u16,
    /* Server status */
    status: Arc<Mutex<Status>>,
    /* TCP socket */
    sock: Arc<TcpListener>
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
            sock: Arc::new(sock)
        };
        tsi.set_timeout(timeout);
        return tsi;
    }
}

impl TServInstance for TcpServInstance{
    /* Test if the server status is RUNNING */
    fn is_running(&self) -> bool{
        return false;
    }

    /* Get the current server status */
    fn get_status(&self) -> Status{
        return Status::STOPED;
    }

    /* Set the socket timeout
        d : timeout duration
    */
    fn set_timeout(&self, d: Duration){

    }

    /* Get the socket timeout */
    fn get_timeout(&self) -> Option<Duration>{
        None
    }

    /* Run a server instance
        tx : mpsc sender where send received datas
    */
    fn run(&mut self, tx: Sender<RecvHandle>){

    }

    /* Stop the running server */
    fn stop(&mut self){

    }

    /* Try to send data through the socket of this server
        addr : target address
        data : data to send (must be shorter than the buffer length)
        return true in case of success
    */
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> bool {
        return false;
    }
}
