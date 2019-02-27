use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Request, Response, Body, StatusCode};

pub fn default_404_handler(_: Request<Body>) -> Response<Body> {
    let body = "page not found";
    make_response(&body, StatusCode::NOT_FOUND)
}

pub fn method_not_supported_handler(_: Request<Body>) -> Response<Body> {
    let body = "method not supported";
    make_response(&body, StatusCode::METHOD_NOT_ALLOWED)
}

pub fn internal_server_error_handler(_: Request<Body>) -> Response<Body> {
    let body = "internal server error";
    make_response(&body, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn not_implemented_handler(_: Request<Body>) -> Response<Body> {
    let body = "not implemented";
    make_response(&body, StatusCode::NOT_IMPLEMENTED)
}

fn make_response(body: &'static str, status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct response")
}