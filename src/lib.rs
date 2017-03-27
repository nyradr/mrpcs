
pub mod server;
mod udpserver;
mod tcpserver;
pub mod rpc;

/* Buffer size */
const BUFFER_SIZE: usize = 1024;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
