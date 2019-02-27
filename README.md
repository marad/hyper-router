# Hyper Router

This cargo is a small extension to the great Hyper HTTP library. It basically is
adds the ability to define routes to request handlers and then query for the handlers
by request path.

[API Documentation](http://radoszewski.pl/hyper-router/0.5.0/hyper_router/)

## Usage

To use the library just add:

```
hyper = "^0.12"
hyper-router = "^0.5"
```

to your dependencies.

```rust
extern crate hyper;
extern crate hyper_router;

use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Request, Response, Body, Method};
use hyper::server::Server;
use hyper::rt::Future;
use hyper_router::{Route, RouterBuilder, RouterService};

fn request_handler(_: Request<Body>) -> Response<Body> {
    let body = "Hello World";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(request_handler))
        .add(Route::from(Method::PATCH, "/asd").using(request_handler))
        .build();

    Ok(RouterService::new(router))
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(router_service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server)
}
```

This code will start Hyper server and add use router to find handlers for request.
We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
will be called.

## Things to note

* you can specify paths as regular expressions so you can match every path you please.
* If you have request matching multiple paths the one that was first `add`ed will be chosen.
* ~~This library is in an early stage of development so there may be breaking changes comming.~~ -
  it seems that the library is quite popular so I'm not going to do compatibility breaking changes.

# Further Development

* add the ability to distinguish requests by query parameters.

# Waiting for your feedback

I've created this little tool to help myself learn Rust and to avoid using big frameworks
like Iron or rustful. I just want to keep things simple.

Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
Please contact me at moriturius at GMail. You may also create an issue at [project's bug tracker](https://github.com/marad/hyper-router/issues).

