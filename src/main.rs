mod http;
mod server;
use http::Method;
use http::Request;
use server::Server;
use std::env;
mod website_handler;
use website_handler::WebsiteHandler;
fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    //let string = String::from("127.0.0.1:8080".to_string());
    //let string_slice = &string[10..]; //string slice
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_owned());
    server.run(WebsiteHandler::new(public_path));
}
