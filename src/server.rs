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
        loop {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    // let res = listener.accept();

                    // if res.is_err() {
                    //     continue;
                    // }
                    // let (stream, addr) = res.unwrap();
                    println!("accept {:?}", stream);
                },
                Err(e) => println!("err: {}", e),
            }
            

        }
    }
}
