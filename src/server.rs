use std::net::TcpListener;

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
    }
}
