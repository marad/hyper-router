extern crate hyper;
extern crate hyper_router;

use hyper_router::*;
use hyper::server::{Request, Response};

fn test_handler(_: Request, res: Response) {
    res.send(b"Hello World").unwrap();
}

#[test]
fn test_api() {
    // given
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(test_handler))
    .build();
    // how to mock request? :c
    //let request = mock_request(Method::Get, "/hello");

    // when
    //let handler = router.find_handler(&request);

    // then
    //assert_eq!(handler, test_handler);
}

