use std::thread;
use std::sync::{Arc};
use std::net::{UdpSocket, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use server::{Status, RecvHandle, TServInstance};
use udpserver::UdpServInstance;
use ::BUFFER_SIZE;

#[derive(Clone)]
pub enum ServMode{
    UDP(UdpServInstance)
}

#[derive(Clone)]
pub struct Rpc{
    insts: HashMap<u16, ServMode>,
    recvh: Sender<RecvHandle>
}

impl Rpc{

    /* Create new RPC server
    */
    pub fn new(tx: Sender<RecvHandle>) -> Rpc{
        let rpc = Rpc{
            insts: HashMap::new(),
            recvh: tx
        };

        return rpc;
    }

    /* Start asynchronous udp server
        Block until the server is RUNNING
        port : UDP port where the server should listen
        return true if the server is started
    */
    pub fn start_udp(&mut self, port: &u16, timeout: Duration) -> bool{
        if !self.insts.contains_key(port){
            let udps = UdpServInstance::new(port.clone(), timeout);
            let mut nudps = udps.clone();
            let tx = self.recvh.clone();

            thread::spawn(move ||{
                nudps.run(tx);
            });

            // wait until the server is RUNNING
            let mut wait = true;
            while wait{
                match udps.get_status(){
                    Status::RUNNING => {wait = false;}
                    _ => {}
                }
            }

            self.insts.insert(port.clone(), ServMode::UDP(udps));
            true
        }else{
            false
        }

    }

    /* Stop a server running on a port
        port : Server port
    */
    pub fn stop(&mut self, port: &u16){
        match self.insts.remove(port){
            Some(srv) =>{
                match srv {
                    ServMode::UDP(mut udps) =>{
                        udps.stop();

                        // wait until the server is STOPED
                        let mut wait = true;
                        while wait{
                            match udps.get_status(){
                                Status::STOPED => {wait = false;}
                                _ => {}
                            }
                        }

                        self.insts.remove(port);
                    }
                }
            }
            None => {}
        }
    }

    /* Send data to a peer
        port : server socket port
        addr : target address
        data : data to send
    */
    pub fn send(&self, port: &u16, addr: SocketAddr, data: Vec<u8>) -> bool{
        match self.insts.get(port){
            Some(srv) =>{
                match srv{
                    &ServMode::UDP(ref udps) =>{
                        return udps.send(addr, data);
                    }
                }
            }
            None => {false}
        }
    }

    /* Test if the port is used by this RPC server */
    pub fn is_used(&self, port: &u16)->bool{
        return self.insts.contains_key(port);
    }

    /* Test if the port is used by UDP */
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

    /* Get the status of a server
        port : server port
        return Some(status) if the server exist
    */
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
