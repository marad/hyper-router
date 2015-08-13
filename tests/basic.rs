extern crate hyper;
extern crate hyper_router;

use hyper_router::router::*;
use hyper::method::Method::{Get,Post};
use hyper::server::{Request, Response};

fn test_handler(_: Request, res: Response) {
    res.send(b"Hello World").unwrap();
}

#[test]
fn test_api() {
    let router = RouterBuilder::new()
        .add(Route {
            method: Get,
            path: Path::new("/hello"),
            handler: test_handler
        })
    .add(Route {
        method: Post,
        path: Path::new("/test"),
        handler: test_handler
    })
    .build();

    println!("{:?}", router);
}

