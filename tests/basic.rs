extern crate hyper;
extern crate hyper_router;

use hyper::{Body, Method, Request, Response, Uri};
use hyper_router::*;
use std::str::FromStr;

#[test]
fn test_get_route_with_static_path() {
    let request = Request::builder()
        .method(Method::GET)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
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

    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(handle_get_hello))
        .add(Route::get("/").using(handle_get_root))
        .add(Route::get("/foo").using(handle_get_foo))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_get_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_post_route_with_static_path() {
    let request = Request::builder()
        .method(Method::POST)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
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

    let router = RouterBuilder::new()
        .add(Route::post("/hello").using(handle_post_hello))
        .add(Route::get("/").using(handle_post_root))
        .add(Route::get("/foo").using(handle_post_foo))
        .add(Route::get("/hello").using(handle_get_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_post_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_delete_route_with_static_path() {
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_delete_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .add(Route::delete("/hello").using(handle_delete_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_delete_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_options_route_with_static_path() {
    let request = Request::builder()
        .method(Method::OPTIONS)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_options_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .add(Route::options("/hello").using(handle_options_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_options_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_put_route_with_static_path() {
    let request = Request::builder()
        .method(Method::PUT)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_put_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .add(Route::put("/hello").using(handle_put_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_put_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_head_route_with_static_path() {
    let request = Request::builder()
        .method(Method::HEAD)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_head_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .add(Route::head("/hello").using(handle_head_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_head_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_trace_route_with_static_path() {
    let request = Request::builder()
        .method(Method::TRACE)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_trace_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .add(Route::trace("/hello").using(handle_trace_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_trace_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_patch_route_with_static_path() {
    let request = Request::builder()
        .method(Method::PATCH)
        .uri(Uri::from_str("http://www.example.com/hello").unwrap())
        .body(Body::empty())
        .unwrap();

    fn handle_patch_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_post_hello(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .add(Route::patch("/hello").using(handle_patch_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_patch_hello as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_route_not_found() {
    let request = Request::builder()
        .method(Method::GET)
        .uri(Uri::from_str("http://www.example.com/notfound").unwrap())
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

    let router = RouterBuilder::new()
        .not_found(handle_not_found)
        .add(Route::patch("/foo").using(handle_get_foo))
        .add(Route::patch("/bar").using(handle_get_bar))
        .build();

    let (handler, params) = router.find_handler(&request);
    assert!(handler as fn(_) -> _ == handle_not_found as fn(_) -> _);
    assert_eq!(params.len(), 0);
}

#[test]
fn test_method_not_supported_with_static_path() {
    fn handle_method_not_supported(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    }
    fn handle_get_foo(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };
    fn handle_get_bar(_: Request<Body>) -> Response<Body> {
        unimplemented!()
    };

    let router = RouterBuilder::new()
        .method_not_supported(handle_method_not_supported)
        .add(Route::patch("/foo").using(handle_get_foo))
        .add(Route::patch("/bar").using(handle_get_bar))
        .add(Route::get("/bar").using(handle_get_bar))
        .build();

    {
        let request = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_str("http://www.example.com/foo").unwrap())
            .body(Body::empty())
            .unwrap();
        let (handler, params) = router.find_handler(&request);
        assert!(handler as fn(_) -> _ == handle_method_not_supported as fn(_) -> _);
        assert_eq!(params.len(), 0);
    }
    {
        // test that the router checks all routes before assuming the method
        // is not supported.
        let request = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_str("http://www.example.com/bar").unwrap())
            .body(Body::empty())
            .unwrap();
        let (handler, params) = router.find_handler(&request);
        assert!(handler as fn(_) -> _ == handle_get_bar as fn(_) -> _);
        assert_eq!(params.len(), 0);
    }
}
