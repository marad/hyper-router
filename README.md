# Hyper Router

This cargo is a small extension to the great Hyper HTTP library. It basically is
adds the ability to define routes to request handlers and then query for the handlers
by request path.

[API Documentation](https://marad.github.io/hyper-router/doc/hyper_router)

## Usage

To use the library just add:

```
hyper-router = "*"
```

to your dependencies.

```rust
extern crate hyper;
extern crate hyper_router;

use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper_router::{Route, RouterBuilder};

fn basic_handler(_: Request, res: Response) {
  res.send(b"Hello World!").unwrap();
}

fn main() {
  let router = RouterBuilder::new()
    .add(Route::get("/greet").using(basic_handler))
    .build();

  Server::http("0.0.0.0:8080").unwrap()
    .handle(move |request: Request, response: Response| {
      match router.find_handler(&request) {
        Ok(handler) => handler(request, response),
        Err(StatusCode::NotFound) => response.send(b"not found").unwrap(),
        Err(_) => response.send(b"some error").unwrap()
      }
    }).unwrap();
}
```

This code will start Hyper server and add use router to find handlers for request.
We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
will be called.

## Things to note

* you can specify paths as regular expressions so you can match every path you please.
* If you have request matching multiple paths the one that was first `add`ed will be chosen.
* This library is in an early stage of development so there may be breaking changes comming
(but I'll try as hard as I can not to break backwards compatibility or break it just a little -
I promise I'll try!).

# Further Development

* add the ability to distinguish requests by query parameters.

# Waiting for your feedback

I've created this little tool to help myself learn Rust and to avoid using big frameworks
like Iron or rustful. I just want to keep things simple.

Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
Please contact me at moriturius at GMail. You may also create an issue at [project's bug tracker](https://github.com/marad/hyper-router/issues).

