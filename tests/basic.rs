extern crate hyper;
extern crate hyper_router;

use hyper_router::*;
use hyper::server::{Request, Response};
use hyper::buffer::BufReader;
use hyper::net::NetworkStream;
use std::io::{self, Read, Write};
use std::time::Duration;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};


struct MockStream {
    read: usize,
    len: usize,
    buf: [u8; 4096],
}


impl MockStream {
    pub fn new(r: &[u8]) -> MockStream {
        let mut stream = MockStream {
            read: 0,
            len: r.len(),
            buf: [0; 4096],
        };
        for (to, from) in stream.buf[..].iter_mut().zip(r.iter()) {
            *to = *from;
        }
        stream
    }
}


impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let available_bytes = self.len - self.read;
        let mut read = 0;
        let from = self.read;
        let to = if available_bytes > buf.len() {
            self.read + buf.len()
        } else {
            self.read + available_bytes
        };
        for (f, t) in self.buf[from..to].iter().zip(buf.iter_mut()) {
            read += 1;
            *t = *f
        }
        self.read += read;
        Ok(read)
    }
}


impl Write for MockStream {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }
    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}


impl NetworkStream for MockStream {
    fn peer_addr(&mut self) -> io::Result<SocketAddr> {
        unimplemented!()
    }
    fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unimplemented!()
    }
    fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unimplemented!()
    }
}


#[test]
fn test_mock_steam() {
    let mut stream = MockStream::new(
        b"GET / HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut reader: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut reader, addr);

    let req = request.unwrap();
    assert_eq!(req.method, hyper::method::Method::Get);
    assert_eq!(req.version, hyper::version::HttpVersion::Http11);
}


#[test]
fn test_get_route() {
    let mut stream = MockStream::new(
        b"GET /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_get_hello (_: Request, _: Response) {};
    fn handle_get_root  (_: Request, _: Response) {};
    fn handle_get_foo   (_: Request, _: Response) {};
    fn handle_post_hello(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(handle_get_hello))
        .add(Route::get("/").using(handle_get_root))
        .add(Route::get("/foo").using(handle_get_foo))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_get_hello as fn(_, _));
}


#[test]
fn test_post_route() {
    let mut stream = MockStream::new(
        b"POST /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_post_hello(_: Request, _: Response) {};
    fn handle_post_root (_: Request, _: Response) {};
    fn handle_post_foo  (_: Request, _: Response) {};
    fn handle_get_hello (_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::post("/hello").using(handle_post_hello))
        .add(Route::post("/").using(handle_post_root))
        .add(Route::post("/foo").using(handle_post_foo))
        .add(Route::get("/hello").using(handle_get_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_post_hello as fn(_, _));
}


#[test]
fn test_delete_route() {
    let mut stream = MockStream::new(
        b"DELETE /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_delete_hello(_: Request, _: Response) {};
    fn handle_post_hello  (_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::delete("/hello").using(handle_delete_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_delete_hello as fn(_, _));
}


#[test]
fn test_options_route() {
    let mut stream = MockStream::new(
        b"OPTIONS /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_options_hello(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::options("/hello").using(handle_options_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_options_hello as fn(_, _));
}


#[test]
fn test_put_route() {
    let mut stream = MockStream::new(
        b"PUT /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_put_hello(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::put("/hello").using(handle_put_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_put_hello as fn(_, _));
}


#[test]
fn test_head_route() {
    let mut stream = MockStream::new(
        b"HEAD /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_head_hello(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::head("/hello").using(handle_head_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_head_hello as fn(_, _));
}


#[test]
fn test_trace_route() {
    let mut stream = MockStream::new(
        b"TRACE /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_trace_hello(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::trace("/hello").using(handle_trace_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_trace_hello as fn(_, _));
}


#[test]
fn test_patch_route() {
    let mut stream = MockStream::new(
        b"PATCH /hello HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_patch_hello(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::patch("/hello").using(handle_patch_hello))
        .build();

    let handler = router.find_handler(&request.unwrap()).unwrap();
    assert!(handler as fn(_, _) == handle_patch_hello as fn(_, _));
}


#[test]
fn test_no_route() {
    let mut stream = MockStream::new(
        b"GET /notfound HTTP/1.1\r\n\
        Host: www.example.com\r\n\
        \r\n\\");

    let mut stream: BufReader<&mut NetworkStream> = BufReader::new(&mut stream);
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 1234));
    let request = Request::new(&mut stream, addr);

    fn handle_get_foo(_: Request, _: Response) {};
    fn handle_get_bar(_: Request, _: Response) {};

    let router = RouterBuilder::new()
        .add(Route::patch("/foo").using(handle_get_foo))
        .add(Route::patch("/bar").using(handle_get_bar))
        .build();

    let handler = router.find_handler(&request.unwrap());

    match handler {
        Ok (_) => panic!("Expected an error, but got a handler instead"),
        Err(e) => assert_eq!(e, hyper::status::StatusCode::NotFound)
    }
}
