use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Request, Response, Body, StatusCode};

pub fn default_404_handler(_: Request<Body>) -> Response<Body> {
    let body = "page not found";
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct response")
}

pub fn method_not_supported_handler(_: Request<Body>) -> Response<Body> {
    let body = "method not supported";
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct response")
}

pub fn internal_server_error_handler(_: Request<Body>) -> Response<Body> {
    let body = "internal server error";
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct response")
}

pub fn not_implemented_handler(_: Request<Body>) -> Response<Body> {
    let body = "not implemented";
    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct response")
}

//fn make_response(body: &str, status: &StatusCode) {
//    Response::new(Body::from(body))
//        .with_status(status)
//        .with_header(ContentLength(body.len() as u64))
//        .with_header(ContentType::plaintext())
//}