fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let server = server::Server::new("127.0.0.1:8000".to_string());
    server.run();
}

mode server {
    pub struct Server {
        addr: String,
    }
    
    impl Server {
    
       pub fn new(addr: String) -> Self {
            Self {
                addr: addr
            }
        }
    
        pub fn run(self) {
            println!("Listening on http://{}", self.addr);
        }
    }
}



struct Request {
    path: String,
    query_string: Option<String>,
    method: String,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}