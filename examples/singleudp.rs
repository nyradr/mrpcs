/* This example create a single UDP listener and draw received datas*/

extern crate rpc;
use rpc::rpc::Rpc;
use rpc::server::RecvHandle;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;

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
    let (tx, rx) = mpsc::channel();
    let mut rpc = Rpc::new(tx);

    println!("{:?}", rpc.start_udp(4242u16, Duration::new(2, 0)));

    let t1 = thread::spawn(move ||{
        req_handler(rx);
    });

    //let data = vec! [0xCA, 0xFE, 0xBA, 0xBE];
    //println!("{:?}", rpc.send(&4242u16, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4242), data));

    rpc.stop(4242u16);
    t1.join();
}
