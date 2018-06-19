extern crate hyper;
extern crate hyper_router;

use hyper_router::*;
use hyper::Body;
use hyper::{Request, Response};
use hyper::Uri;
use std::str::FromStr;


#[test]
fn test_get_route() {
    let request = Request::get(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_get_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_get_root(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_get_foo(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(handle_get_hello))
        .add(Route::get("/").using(handle_get_root))
        .add(Route::get("/foo").using(handle_get_foo))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _ , handle_get_hello as fn(_) -> _);
}


#[test]
fn test_post_route() {
    let request = Request::post(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_root(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_foo(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_get_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::post("/hello").using(handle_post_hello))
        .add(Route::get("/").using(handle_post_root))
        .add(Route::get("/foo").using(handle_post_foo))
        .add(Route::get("/hello").using(handle_get_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_post_hello as fn(_) -> _);
}


#[test]
fn test_delete_route() {
    let request = Request::delete(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_delete_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::delete("/hello").using(handle_delete_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_delete_hello as fn(_) -> _);
}


#[test]
fn test_options_route() {
    let request = Request::options(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_options_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::options("/hello").using(handle_options_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_options_hello as fn(_) -> _);
}


#[test]
fn test_put_route() {
    let request = Request::put(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_put_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::put("/hello").using(handle_put_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_put_hello as fn(_) -> _);
}


#[test]
fn test_head_route() {
    let request = Request::head(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_head_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::head("/hello").using(handle_head_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_head_hello as fn(_) -> _);
}


#[test]
fn test_trace_route() {
    let request = Request::trace(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_trace_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::trace("/hello").using(handle_trace_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_trace_hello as fn(_) -> _);
}


#[test]
fn test_patch_route() {
    let request = Request::patch(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_patch_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::patch("/hello").using(handle_patch_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_patch_hello as fn(_) -> _);
}


#[test]
fn test_no_route() {
    let request = Request::get(Uri::from_str("http://www.example.com/notfound").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_get_foo(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_get_bar(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::patch("/foo").using(handle_get_foo))
        .add(Route::patch("/bar").using(handle_get_bar))
        .build();

    let handler = router.find_handler(&request);

    match handler {
        Ok(_) => panic!("Expected an error, but got a handler instead"),
        Err(e) => assert_eq!(e, hyper::StatusCode::NOT_FOUND)
    }
}


#[test]
fn test_regex_path() {
    let request = Request::get(Uri::from_str("http://www.example.com/foo/bar").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_regex_foo(_: Request<Body>) -> Response<Body> { unimplemented!() };
    fn handle_regex_bar(_: Request<Body>) -> Response<Body> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::get(r"/foo/.*?").using(handle_regex_foo))
        .add(Route::get(r"/bar/.*?").using(handle_regex_bar))
        .build();

    let handler = router.find_handler(&request).unwrap();
    assert_eq!(handler as fn(_) -> _, handle_regex_foo as fn(_) -> _);
}
