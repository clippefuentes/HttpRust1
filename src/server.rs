use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self) {
        println!("server: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Value: {}", String::from_utf8_lossy(&buffer));
                            Request::try_from(&buffer[..] as &[u8]);
                            let res: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(e) => println!("Error: {}", e),
                    }
                    println!("accept {:?}", stream);
                },
                Err(e) => println!("err: {}", e),
            }
            

        }
    }
}
