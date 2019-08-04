#[macro_use]
extern crate criterion;

use criterion::Criterion;
use hyper::*;
use hyper_router::*;
use std::str::FromStr;

fn handler(_: Request<Body>) -> Response<Body> {
    unimplemented!()
}

fn create_router() -> Router {
    RouterBuilder::new()
        .add(Route::get("/a/:id").using(handler))
        .add(Route::get("/b/:id").using(handler))
        .add(Route::get("/c/:id").using(handler))
        .add(Route::get("/d/:id").using(handler))
        .add(Route::get("/e/:id").using(handler))
        .add(Route::get("/f/:id").using(handler))
        .add(Route::get("/a/:b/:c/:d/:e/:f/:g/:h/:i/:g/:h/:i/:j/k/l/:m").using(handler))
        .add(Route::get("/g/:id").using(handler))
        .add(Route::get("/h/:id").using(handler))
        .add(Route::get("/a/:id").using(handler))
        .add(Route::get("/static/route").using(handler))
        .build()
}

fn run_benchmark(c: &mut Criterion) {
    c.bench_function("capture many parameters", |bench| {
        let router = create_router();
        bench.iter(move || {
            let url = "http://a.com/a/b/c/d/e/f/g/h/i/g/h/i/j/k/l/m";
            let req_uri = Uri::from_str(url).unwrap();
            let request = Request::builder()
                .method(Method::GET)
                .uri(req_uri)
                .body(Body::empty())
                .unwrap();
            router.find_handler(&request)
        })
    });

    c.bench_function("capture single parameter", |bench| {
        let router = create_router();
        bench.iter(move || {
            let url = "http://a.com/a/09879182c79isdcnvwevbyqw8e7vby2873rvby283rbvwqrb283r7238rcb2378brc2387bcrq283bcr823rc283rbc2387vbgr238crbg";
            let req_uri = Uri::from_str(url).unwrap();
            let request = Request::builder()
                .method(Method::GET)
                .uri(req_uri)
                .body(Body::empty())
                .unwrap();
            router.find_handler(&request)
        })
    });

    c.bench_function("match static route", |bench| {
        let router = create_router();
        bench.iter(move || {
            let url = "http://a.com/static/route";
            let req_uri = Uri::from_str(url).unwrap();
            let request = Request::builder()
                .method(Method::GET)
                .uri(req_uri)
                .body(Body::empty())
                .unwrap();
            router.find_handler(&request)
        })
    });
}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);
