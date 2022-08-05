use std::option::Option;

use server::Server;
use http::request::Request;


mod server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}



mod http {

    pub mod request{
            use super::method::Method;
            pub struct Request {
                    path: String,
                    query_string: Option<String>,
                    method: Method,
                }
        }
    
    pub mod method{
            pub  enum Method {
                    GET,
                    POST,
                    DELETE,
                    PUT,
                    HEAD,
                    CONNECT,
                    OPTIONS,
                    TRACE,
                    PATH,
                }
        }
}
