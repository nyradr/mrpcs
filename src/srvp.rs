use std::thread;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::time::Duration;
use std::io::{ErrorKind, Error};

use server::{Status, RecvHandle, TServInstance};
use udpserver::UdpServInstance;

#[derive(Clone)]
enum ServMode{
    UDP(UdpServInstance)
}

/// A pool of servers
#[derive(Clone)]
pub struct ServerPool{
    insts: HashMap<u16, ServMode>,
    recvh: Sender<RecvHandle>
}

impl ServerPool{

    /// Create new server pool
    /// # Arguments
    /// * `tx` - MPSC sender
    pub fn new(tx: Sender<RecvHandle>) -> ServerPool{
        ServerPool{
            insts: HashMap::new(),
            recvh: tx
        }
    }

    /// Start asynchronous udp server.
    /// This function will block until the server is `Status::RUNNING`.
    /// Return true if the server is started.
    /// # Arguments
    /// * `port` - UDP port where the server should listen
    /// * `timeout` - Read/Write timeout
    /// * `v4` - Listen for ipv4 address instead of ipv6
    /// * `blen` - Buffer max length
    pub fn start_udp(&mut self, port: u16, timeout: Duration, v4: bool, blen: usize) -> bool{
        if !self.insts.contains_key(&port){
            let udps = UdpServInstance::new(port, timeout, v4, blen);
            let mut nudps = udps.clone();
            let tx = self.recvh.clone();

            thread::spawn(move ||{
                nudps.run(tx);
            });

            // wait until the server is RUNNING
            let mut wait = true;
            while wait{
                match udps.get_status(){
                    Status::Running => {wait = false;}
                    _ => {}
                }
            }

            self.insts.insert(port.clone(), ServMode::UDP(udps));
            true
        }else{
            false
        }

    }

    /// Stop a server running on a port
    /// # Argments
    /// * `port` - Server port
    pub fn stop(&mut self, port: u16){
        match self.insts.remove(&port){
            Some(srv) =>{
                match srv {
                    ServMode::UDP(mut udps) =>{
                        udps.stop();

                        // wait until the server is STOPED
                        let mut wait = true;
                        while wait{
                            match udps.get_status(){
                                Status::Stoped => {wait = false;}
                                _ => {}
                            }
                        }

                        self.insts.remove(&port);
                    }
                }
            }
            None => {}
        }
    }

    /// Stop all running server on the pool
    pub fn stop_all(&mut self){
        let mut ports = vec!();
        for port in self.insts.keys(){
            ports.push(*port);
        }

        for port in ports{
            self.stop(port);
        }
    }

    /// Send data to a peer
    /// # Arguments
    /// * `port` - server socket port
    /// * `addr` - target address
    /// * `data` - data to send
    pub fn send(&self, port: &u16, addr: SocketAddr, data: Vec<u8>) -> Result<usize, Error>{
        match self.insts.get(port){
            Some(srv) =>{
                match srv{
                    &ServMode::UDP(ref udps) => udps.send(addr, data)
                }
            }
            None => Err(Error::new(ErrorKind::NotFound, "unused port by the pool"))
        }
    }

    /// Test if the port is used by this RPC server
    pub fn is_used(&self, port: &u16)->bool{
        self.insts.contains_key(port)
    }

    /// Test if the port is used by UDP
    pub fn is_udp(&self, port: &u16)->bool{
        match self.insts.get(port){
            Some(srv) =>{
                match srv{
                    &ServMode::UDP(_) =>{
                        true
                    }
                }
            }
            None =>{
                false
            }
        }
    }

    /// Get the status of a server
    /// return Some(status) if the server exist
    /// # Arguments
    /// * `port` - server port
    pub fn status_of(&self, port: &u16)->Option<Status>{
        match self.insts.get(port){
            Some(srv) =>{
                match srv{
                    &ServMode::UDP(ref u) =>{
                        Some(u.get_status())
                    }
                }
            }
            None =>{
                None
            }
        }
    }
}
