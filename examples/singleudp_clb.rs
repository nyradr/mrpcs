/* This example create a single UDP listener (for ip v4 and v6) and print the received datas.
    This example use callbacks to avoid user mpsc and thread management
    You can use something like ncat (on GNU/Linux) to send the datas
*/
extern crate mrpcs;

use std::net::SocketAddr;
use mrpcs::srvp::ServerPool;
use mrpcs::server::{Status, RecvMess};

use std::time::Duration;
use std::io::{self, Read};

// handle message received
fn mess_handler(rm: RecvMess){
    println!("Packet received");
    println!("Port : {:?}", rm.get_port());
    println!("From : {:?}", rm.get_addr());
    println!("Time : {:?}", rm.get_time());
    println!("Data : {:?}", rm.get_data());
}

// handle message timeout
fn timeout_handler(addr: SocketAddr){
    println!("Client timeout {:?}", addr);
}

const PORT4: u16 = 4242;
const PORT6: u16 = 4243;
const BUFFER_SIZE: usize = 1024;

fn main() {
    // rpc server manager
    let mut pool = ServerPool::new_callbacks(mess_handler, timeout_handler);

    // start an asynchronous UDP listener of port 4242, with timeout of 2s
    pool.start_udp(PORT4, Duration::new(2, 0), true, BUFFER_SIZE);
    pool.start_udp(PORT6, Duration::new(2, 0), false, BUFFER_SIZE);

    // wait until the user want to stop
    let mut inp = String::new();
    while inp != "stop\n"{
        inp.clear();
        io::stdin().read_line(&mut inp);
    }

    // stop the server
    pool.stop_all();
}
