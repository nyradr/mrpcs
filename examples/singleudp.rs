/* This example create a single UDP listener and draw the received datas
    You can use something like ncat (on GNU/Linux) to send the datas
*/

extern crate mrpcs;
use mrpcs::rpc::Rpc;
use mrpcs::server::{Status, RecvHandle};

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use std::io::{self, Read};

fn req_handler(rx: Receiver<RecvHandle>){
    for rh in rx.iter(){
        println!("Packet received");
        println!("Port : {:?}", rh.get_port());
        println!("From : {:?}", rh.get_addr());
        println!("Time : {:?}", rh.get_time());
        println!("Data : {:?}", rh.get_data());
    }
}

fn main() {
    // data handler
    let (tx, rx) = mpsc::channel();

    // rpc server manager
    let mut rpc = Rpc::new(tx);

    // start an asynchronous UDP listener of port 4242, with timeout of 2s
    rpc.start_udp(4242u16, Duration::new(2, 0));

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
    rpc.stop(4242u16);
}
