use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Sender};

/// Server status
#[derive(Clone, Debug)]
pub enum Status{
    /// The server is starting but not yet running
    STARTING,
    /// The server is running
    RUNNING,
    /// The server is stopping but not yes stopped
    STOPING,
    /// The server is stopped
    STOPED
}

/// Signal throwed by a server
pub enum RecvHandle{
    /// The server receive a message
    Mess(RecvMess),
    /// Some client has timeout
    Timeout(SocketAddr)
}


/// Data throwed when a message is received
pub struct RecvMess{
    /* Server port */
    port: u16,
    /* Client address */
    addr: SocketAddr,
    /* Message arrival time */
    time: Instant,
    /* Message data */
    data: Vec<u8>
}

impl RecvMess{
    /// Create a new RecvHandle and initialize it
    /// Set time to now
    /// # Arguments
    /// * `port` : server port
    /// * `addr` : client address
    /// * `data` : data received
    pub fn new(port: u16, addr: SocketAddr, data: Vec<u8>) -> RecvMess{
        let rh = RecvMess {
            port: port,
            addr: addr,
            time: Instant::now(),
            data: data
        };
        return rh;
    }

    /// Get port where the message come from
    pub fn get_port(&self) -> &u16{
        return &self.port;
    }

    /// Get the client inet address
    pub fn get_addr(&self) -> &SocketAddr{
        return &self.addr;
    }

    /// Get the message arrival time
    pub fn get_time(&self) -> &Instant{
        return &self.time;
    }

    /// Get the message received
    pub fn get_data(&self) -> &Vec<u8>{
        return &self.data;
    }
}

/// Interfaces functions to be implemented by any ServInstance
pub trait TServInstance{
    /// Test if the server status is `Status::RUNNING`
    fn is_running(&self) -> bool;

    /// Get the current server status
    fn get_status(&self) -> Status;

    /// Set the socket timeout
    /// # Arguments
    /// * `d` : timeout duration
    fn set_timeout(&self, d: Duration);

    /// Get the socket timeout
    fn get_timeout(&self) -> Duration;

    /// Run a server instance
    /// # Arguments
    /// * `tx` : mpsc sender where send received datas
    fn run(&mut self, tx: Sender<RecvHandle>);

    /// Stop the running server
    fn stop(&mut self);

    /// Try to send data through the socket of this server.
    /// return true in case of success.
    /// # Arguments
    /// * `addr` : target address
    /// * `data` : data to send (must be shorter than the buffer length)
    fn send(&self, addr: SocketAddr, data:Vec<u8>) -> bool;
}
