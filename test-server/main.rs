extern crate hyper;
extern crate hyper_router;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response};
use hyper::Method;
use hyper_router::{Route, RouterBuilder, RouterService};

fn request_handler(_: Request) -> Response {
    let body = "Hello World";
    Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_header(ContentType::plaintext())
        .with_body(body)
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(request_handler))
        .add(Route::from(Method::Patch, "/asd").using(request_handler))
        .build();

    Ok(RouterService::new(router))
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Http::new().bind(&addr, router_service).unwrap();
    server.run().unwrap();
}
