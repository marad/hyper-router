extern crate hyper;
extern crate hyper_router;

use hyper::{Body, Method, Request, Response, Uri};
use hyper_router::*;
use std::str::FromStr;

#[test]
fn test_get_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::GET)
        .uri(Uri::from_str("http://www.example.com/hello/123").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_get_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_get_root(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_get_foo(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::get("/hello/:id", handle_get_hello))
        .add(Route::get("/foo/:id", handle_get_root))
        .add(Route::get("/foo", handle_get_foo))
        .add(Route::post("/hello", handle_post_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_get_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["123"]);
}

#[test]
fn test_post_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::POST)
        .uri(
            Uri::from_str("http://www.example.com/hello/5ea67884-245e-43f8-80f7-91d00971a562")
                .unwrap(),
        )
        .body(Body::empty())
        .unwrap();

    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_root(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_foo(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_get_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::post("/hello/:id", handle_post_hello))
        .add(Route::get("/", handle_post_root))
        .add(Route::get("/foo", handle_post_foo))
        .add(Route::get("/hello", handle_get_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_post_hello as fn(_) -> _);
    assert_eq!(
        params.parameters,
        vec!["5ea67884-245e-43f8-80f7-91d00971a562"]
    );
}

#[test]
fn test_delete_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(Uri::from_str("http://www.example.com/hello/my-hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_delete_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::delete("/hello/:id", handle_delete_hello))
        .add(Route::post("/hello", handle_post_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_delete_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["my-hello"]);
}

#[test]
fn test_options_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::OPTIONS)
        .uri(Uri::from_str("http://www.example.com/hello/7777").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_options_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::options("/hello/:id", handle_options_hello))
        .add(Route::post("/hello/:id", handle_post_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_options_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["7777"]);
}

#[test]
fn test_put_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::PUT)
        .uri(Uri::from_str("http://www.example.com/hello/deadbeef").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_put_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::put("/hello/:id", handle_put_hello))
        .add(Route::post("/hello/:id", handle_post_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_put_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["deadbeef"]);
}

#[test]
fn test_head_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::HEAD)
        .uri(Uri::from_str("http://www.example.com/hello/goodbye").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_head_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::head("/hello/:id", handle_head_hello))
        .add(Route::post("/hello/:id", handle_post_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_head_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["goodbye"]);
}

#[test]
fn test_trace_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::TRACE)
        .uri(Uri::from_str("http://www.example.com/hello/hi").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_trace_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::trace(
            "/hello/:something-very-long",
            handle_trace_hello,
        ))
        .add(Route::post(
            "/hello/:something-very-long",
            handle_post_hello,
        ));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_trace_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["hi"]);
}

#[test]
fn test_patch_route_with_parametric_path() {
    let request = Request::builder()
        .method(Method::PATCH)
        .uri(Uri::from_str("http://www.example.com/hello/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_patch_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .add(Route::patch("/hello/:hello", handle_patch_hello))
        .add(Route::post("/hello/:hello", handle_post_hello));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_patch_hello as fn(_) -> _);
    assert_eq!(params.parameters, vec!["hello"]);
}

#[test]
fn test_route_not_found() {
    let request = Request::builder()
        .method(Method::GET)
        .uri(Uri::from_str("http://www.example.com/notfound/77").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_not_found(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    }
    fn handle_get_foo(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_get_bar(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = Router::new()
        .not_found(handle_not_found)
        .add(Route::patch("/foo/:id", handle_get_foo))
        .add(Route::patch("/bar/:id", handle_get_bar));

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_not_found as fn(_) -> _);
    assert_eq!(params.parameters, vec![] as Vec<String>);
}
