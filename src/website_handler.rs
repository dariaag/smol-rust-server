use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        //Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_owned()))
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}