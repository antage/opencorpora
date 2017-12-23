extern crate bzip2;
extern crate futures;
extern crate hyper;
extern crate opencorpora;
extern crate tokio_core;

use std::io::Cursor;

use hyper::{Client, Method, Request, StatusCode};
use hyper::header::Connection;
use bzip2::read::BzDecoder;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

use opencorpora::Dict;

#[test]
fn test_read_from_xml() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let uri = "http://opencorpora.org/files/export/dict/dict.opcorpora.xml.bz2"
        .parse()
        .unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(Connection::close());
    let task = client.request(req).and_then(|res| {
        assert_eq!(res.status(), StatusCode::Ok);
        res.body().concat2()
    });

    let body = core.run(task).unwrap();
    let decompressed = BzDecoder::new(Cursor::new(&body));

    let dict = Dict::read_from_xml(decompressed);
    assert!(dict.is_ok());
}
