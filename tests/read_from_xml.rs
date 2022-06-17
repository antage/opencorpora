use std::io::Cursor;

use hyper::{Client, Uri, StatusCode};
use bzip2::read::BzDecoder;

use opencorpora::Dict;

#[test]
fn test_read_from_xml() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let client = Client::new();
            let uri = Uri::from_static("http://opencorpora.org/files/export/dict/dict.opcorpora.xml.bz2");
            let res = client.get(uri).await.unwrap();
            assert_eq!(res.status(), StatusCode::OK);
            let bytes = hyper::body::to_bytes(res).await.unwrap();

            let decompressed = BzDecoder::new(Cursor::new(bytes));

            let dict = Dict::read_from_xml(decompressed);
            assert!(dict.is_ok());
        });
}
