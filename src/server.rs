use crate::http::Request;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::net::TcpListener;

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
                Ok((mut tcp_stream, _)) => {
                    let mut buffer = [0; 1024];
                    match tcp_stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));
                            //Request::try_from(&buffer[..]);
                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {}
                                Err(err) => println!("Failed to parse the request {}", err),
                            }
                            // let res:&Result<Request,_> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Error While Reading data: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
