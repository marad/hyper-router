extern crate hyper;
extern crate rustc_serialize;

mod router;

use std::io;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method::Get;
use rustc_serialize::json;

use router::{Route, Router, RouterBuilder};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    name: String,
    age: i32,
}

fn request_handler(req: Request, mut res: Response) {
        let person = Person { 
            name: "Stefan".to_string(), 
            age: 45,
        };

        res.send(&mut json::encode(&person).unwrap().as_bytes()).unwrap();

        //io::copy(
        //    &mut json::encode(&person).unwrap().as_bytes(),
        //    &mut res.start().unwrap()
        //).unwrap();
}

fn main() {
    let router = RouterBuilder::new()
        .add(Route {
            method: Get,
            path: "/hello".to_string(),
            handler: request_handler
        }).build();
    
    Server::http("0.0.0.0:8080").unwrap()
        .handle(move |request: Request, response: Response| {
            let handler = router.find_handler(&request.uri);
            handler(request, response);
        }).unwrap();
}
