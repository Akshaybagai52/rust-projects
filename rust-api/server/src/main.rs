fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {

    fn new(addr: String) -> Self {
        Self {
            addr: addr
        }
    }

    fn run(self) {
        println!("Listening on http://{}", self.addr);
    }
}