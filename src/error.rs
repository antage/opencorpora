error_chain! {
    links {
        XmlError(::quick_xml::errors::Error, ::quick_xml::errors::ErrorKind);
    }

    foreign_links {
        Utf8Error(::std::str::Utf8Error);
        ParseIntError(::std::num::ParseIntError);
    }

    errors {
        ParsingError(err: String) {
            description("XML dictionary parsing error"),
            display("XML dictionary parsing error: {}", err),
        }
    }
}
