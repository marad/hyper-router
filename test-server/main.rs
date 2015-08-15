extern crate hyper;
extern crate hyper_router;

use hyper::server::{Server, Request, Response};
use hyper::method::Method;
use hyper_router::{Route, RouterBuilder};

fn request_handler(_: Request, res: Response) {
    res.send(b"Hello World").unwrap();
}

fn main() {
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(request_handler))
        .add(Route::from(Method::Patch, "/asd").using(request_handler))
        .build();
    
    Server::http("0.0.0.0:8080").unwrap()
        .handle(move |request: Request, response: Response| {
            let handler = router.find_handler_with_defaults(&request);
            handler(request, response);
        }).unwrap();
}
