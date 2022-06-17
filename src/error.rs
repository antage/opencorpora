use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("XML parser error")]
    Xml {
        #[from]
        source: quick_xml::Error,
    },

    #[error("XML attribute parser error")]
    XmlAttr {
        #[from]
        source: quick_xml::events::attributes::AttrError,
    },

    #[error("UTF-8 encoding error")]
    Utf8 {
        #[from]
        source: std::str::Utf8Error,
    },

    #[error("Integer parsing error")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("XML dictionary parsing error: {0}")]
    Parsing(String),
}

pub type Result<R> = std::result::Result<R, Error>;
