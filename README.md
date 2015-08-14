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
use hyper::method::Method::Get;
use hyper_router::{Route, RouterBuilder, Path};

fn basic_handler(_: Request, res: Response) {
  res.send(b"Hello World!").unwrap();
}

fn main() {
  let router = RouterBuilder::new()
    .add(Route {
      method: Get,
      path: Path::new("/greet"),
      handler: basic_handler
    })
    .build();

  Server::http("0.0.0.0:8080").unwrap()
    .handle(move |request: Request, response: Response| {
      let handler = router.find_handler(&request);
      handler(request, response);
    }).unwrap();
}
```

This code will start Hyper server and add use router to find handlers for request.
We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
will be called.

## Things to note

* `Path::new` method accepts regular expressions so you can match every path you please.
* If you have request matching multiple paths the one that was first `add`ed will be chosen.
* This library is in an early stage of development so there may be breaking changes comming
(but I'll try as hard as I can not to break backwards compatibility or break it just a little -
I promise I'll try!).

# Further Development

* add the ability to distinguish requests by query parameters.
* maybe some small API changes/upgrades

# Waiting for your feedback

I've created this little tool to help myself learn Rust and to avoid using big frameworks
like Iron or rustful. I just want to keep things simple.

Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
Please contact me at moriturius at GMail. You may also create an issue at [project's bug tracker](https://github.com/marad/hyper-router/issues).
