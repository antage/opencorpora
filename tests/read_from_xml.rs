extern crate hyper;
extern crate bzip2;
extern crate opencorpora;

use hyper::Client;
use hyper::header::Connection;

use bzip2::read::BzDecoder;

use opencorpora::Dict;

#[test]
fn test_read_from_xml() {
    let client = Client::new();
    let res = client
        .get("http://opencorpora.org/files/export/dict/dict.opcorpora.xml.bz2")
        .header(Connection::close())
        .send()
        .unwrap();
    let decompressed = BzDecoder::new(res);

    let dict = Dict::read_from_xml(decompressed);
    assert!(dict.is_ok());
}
