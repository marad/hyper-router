extern crate hyper;
extern crate hyper_router;
extern crate rustc_serialize;

use std::io::prelude::*;
use hyper::server::{Server, Request, Response};
use hyper::method::Method::{Get,Post};
use rustc_serialize::json;
use hyper_router::{Route, RouterBuilder, Path};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    name: String,
    age: i32,
}

fn request_handler(_: Request, res: Response) {
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

fn echo_handler(mut req: Request, res: Response) {
    let mut body = String::new();
    req.read_to_string(&mut body).unwrap();
    res.send(body.as_bytes()).unwrap();
}

fn main() {
    let router = RouterBuilder::new()
        //.add(Route::get("/hello")
        //     .with_params()
        //     .using(request_handler))
        .add(Route {
            method: Get,
            path: Path::new("/hello"),
            handler: request_handler
        })
        .add(Route {
            method: Post,
            path: Path::new(r"/echo/\d{1,5}"),
            handler: echo_handler
        })
        .build();
    
    Server::http("0.0.0.0:8080").unwrap()
        .handle(move |request: Request, response: Response| {
            let handler = router.find_handler(&request);
            handler(request, response);
        }).unwrap();
}
