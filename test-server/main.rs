extern crate futures;
extern crate hyper;
extern crate hyper_router;

use futures::future::FutureResult;
use hyper::server::{Http, Service, Request, Response};
use hyper::StatusCode;
use hyper::Method;
use hyper::header::{ContentLength, ContentType};
use hyper_router::{Route, Router, RouterBuilder};

fn request_handler(_: Request) -> Response {
    let body = "Hello World";
    Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_header(ContentType::plaintext())
        .with_body(body)
}

struct RouterService {
    router: Router
}

impl RouterService {
    fn new() -> RouterService {
        let router = RouterBuilder::new()
            .add(Route::get("/hello").using(request_handler))
            .add(Route::from(Method::Patch, "/asd").using(request_handler))
            .build();
        RouterService {
            router
        }
    }
}

impl Service for RouterService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, request: Request) -> Self::Future {
        futures::future::ok(
            match self.router.find_handler(&request) {
                Ok(handler) => handler(request),
                Err(StatusCode::NotFound) => Response::new().with_status(StatusCode::NotFound),
                Err(_) => {
                    let error = "some error";
                    Response::new()
                        .with_status(StatusCode::InternalServerError)
                        .with_header(ContentLength(error.len() as u64))
                        .with_body(error)
                }
            }
        )
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Http::new().bind(&addr, || { Ok(RouterService::new()) }).unwrap();
    server.run().unwrap();
}
