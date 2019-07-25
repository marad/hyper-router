extern crate hyper;
extern crate hyper_router;

use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::rt::Future;
use hyper::server::Server;
use hyper::{Body, Method, Request, Response};
use hyper_router::{Route, RouteParameters, Router, RouterService};

fn request_handler(_: Request<Body>) -> Response<Body> {
    let body = "Hello World";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn greeting_handler(req: Request<Body>) -> Response<Body> {
    let params: &RouteParameters = req.extensions().get().unwrap();
    let name = params.parameters.get(0).unwrap();
    let body = format!("Hello, {}!", name);
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = Router::new()
        .add(Route::get("/hello", request_handler))
        .add(Route::get("/greeting/:name", greeting_handler))
        .add(Route::from(Method::PATCH, "/world", request_handler));

    Ok(RouterService::new(router))
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(router_service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server)
}
