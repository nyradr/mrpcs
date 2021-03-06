/* This example create a single UDP listener and draw the received datas
    You can use something like ncat (on GNU/Linux) to send the datas
*/

extern crate mrpcs;
use mrpcs::srvp::ServerPool;
use mrpcs::server::{Status, RecvHandle};

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use std::io::{self, Read};

fn req_handler(rx: Receiver<RecvHandle>){
    for rh in rx.iter(){
        match rh{
            RecvHandle::Mess(rm) => {
                println!("Packet received");
                println!("Port : {:?}", rm.get_port());
                println!("From : {:?}", rm.get_addr());
                println!("Time : {:?}", rm.get_time());
                println!("Data : {:?}", rm.get_data());
            }
            RecvHandle::Timeout(addr) =>{
                println!("Client timeout {:?}", addr);
            }
        }
    }
}

const PORT4: u16 = 4242;
const PORT6: u16 = 4243;
const BUFFER_SIZE: usize = 1024;

fn main() {
    // data handler
    let (tx, rx) = mpsc::channel();

    // rpc server manager
    let mut pool = ServerPool::new(tx);

    // start an asynchronous UDP listener of port 4242, with timeout of 2s
    pool.start_udp(PORT4, Duration::new(2, 0), true, BUFFER_SIZE);
    pool.start_udp(PORT6, Duration::new(2, 0), false, BUFFER_SIZE);

    // start data handler
    let t = thread::spawn(move ||{
        req_handler(rx);
    });

    // wait until the user want to stop
    let mut inp = String::new();
    while inp != "stop\n"{
        inp.clear();
        io::stdin().read_line(&mut inp);
    }

    // stop the server
    pool.stop_all();
}
