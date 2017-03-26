use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Sender};
use std::fmt;

/* Server status */
#[derive(Clone, Debug)]
pub enum Status{
    /* The server is starting but not yet running */
    STARTING,
    /* The server is running */
    RUNNING,
    /* The server is stopping but not yes stopped */
    STOPING,
    /* The server is stopped */
    STOPED
}


/* Data throwed when a message is received
*/
pub struct RecvHandle{
    /* Server port */
    port: u16,
    /* Client address */
    addr: SocketAddr,
    /* Message arrival time */
    time: Instant,
    /* Message data */
    data: Vec<u8>
}

impl RecvHandle{
    /* Create a new RecvHandle and initialize it
        Set time to now
    */
    pub fn new(port: u16, addr: SocketAddr, data: Vec<u8>) -> RecvHandle{
        let rh = RecvHandle {
            port: port,
            addr: addr,
            time: Instant::now(),
            data: data
        };
        return rh;
    }

    pub fn get_port(&self) -> &u16{
        return &self.port;
    }

    pub fn get_addr(&self) -> &SocketAddr{
        return &self.addr;
    }

    pub fn get_time(&self) -> &Instant{
        return &self.time;
    }

    pub fn get_data(&self) -> &Vec<u8>{
        return &self.data;
    }
}

/* Interfaces functions to be implemented by any ServInstance */
pub trait TServInstance{
    /* Test if the server status is RUNNING */
    fn is_running(&self) -> bool;

    /* Get the current server status */
    fn get_status(&self) -> Status;

    /* Set the socket timeout
        d : timeout duration
    */
    fn set_timeout(&self, d: Duration);

    /* Get the socket timeout */
    fn get_timeout(&self) -> Option<Duration>;

    /* Run a server instance
        tx : mpsc sender where send received datas
    */
    fn run(&mut self, tx: Sender<RecvHandle>);

    /* Stop the running server */
    fn stop(&mut self);

    /* Try to send data through the socket of this server
        addr : target address
        data : data to send (must be shorter than the buffer length)
        return true in case of success
    */
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> bool;
}
