use std::io::Read;
use std::net::{TcpListener};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }
    pub fn run(self) {
        println!("Server is listening on {}", &self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut tcp_stream, socket_addr)) => {
                    println!("Established connection {socket_addr}");
                    let mut buffer = [0; 1024];
                    match tcp_stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("Error While Reading data: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
