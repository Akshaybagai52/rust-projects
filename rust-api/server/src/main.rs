use http::request::Request;
mod server;

fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
    let server = server::Server::new("127.0.0.1:8000".to_string());
    server.run();
}


mod http {
   pub mod request {
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }

   pub mod method {
        pub enum Method {
            GET,
            POST,
            PUT,
            DELETE,
            PATCH,
        }
    }
}



