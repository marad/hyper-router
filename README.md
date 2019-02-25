# Hyper Router

This cargo is a small extension to the great Hyper HTTP library. It basically is
adds the ability to define routes to request handlers and then query for the handlers
by request path.

[API Documentation](http://radoszewski.pl/hyper-router/0.4.0/hyper_router/)

## Usage

To use the library just add:

```
hyper-router = "*"
```

to your dependencies.

```rust
extern crate hyper;
extern crate hyper_router;

use hyper::server::{Http, Request, Response};
use hyper::header::{ContentLength, ContentType};
use hyper_router::{Route, RouterBuilder, RouterService};
use std::ops::Add;

fn basic_handler(_: Request) -> Response {
    let body = "Hello World";
    Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_header(ContentType::plaintext())
        .with_body(body)
}

fn router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/greet").using(basic_handler))
        .build();

    Ok(RouterService::new(router))
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Http::new().bind(&addr, router_service).unwrap();
    server.run().unwrap();
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

