//! Данная библиотека предназначена для работы со словарём
//! проекта [opencorpora.org](http://opencorpora.org).
//!
//! # Examples
//!
//! ## Чтение XML-словаря
//!
//! ```no_run
//! extern crate opencorpora;
//!
//! use opencorpora::Dict;
//!
//! fn main() {
//!     use std::fs::File;
//!
//!     let file = File::open("dict.xml").unwrap();
//!     match Dict::read_from_xml(&file) {
//!         Ok(dict) => {
//!             println!("Version: {}", dict.version);
//!             println!("Revision: {}", dict.revision);
//!             println!("Grammemes count: {}", dict.grammemes.len());
//!             println!("Restrictions count: {}", dict.restrictions.len());
//!             println!("Lemmata count: {}", dict.lemmata.len());
//!             println!("Link types count: {}", dict.link_kinds.len());
//!             println!("Links count: {}", dict.links.len());
//!         },
//!         Err(e) => {
//!             println!("ERROR: {}", e);
//!         },
//!     }
//! }
//! ```

#[macro_use] extern crate quick_error;
extern crate quick_xml;

mod dict;

pub use dict::{
    Dict,
    Grammeme,
    Restriction,
    RestrictionScope,
    RestrictionKind,
    Lemma,
    Form,
    LinkKind,
    Link,
};

quick_error! {
    /// Перечисление инкапсулирующие различные виды ошибок,
    /// которые могут возникать при работе со словарём.
    #[derive(Debug)]
    pub enum DictError {
        /// Ошибка разбора XML.
        XmlError { err: ::quick_xml::error::Error, pos: usize } {
            from(e: (::quick_xml::error::Error, usize)) -> {
                err: e.0,
                pos: e.1
            }
            cause(err)
            description(err.description())
            display("XML error at {} position: {}", pos, err)
        }

        /// Текст не соответствует кодировке UTF-8.
        Utf8Error(err: ::std::str::Utf8Error) {
            from(e: ::std::str::Utf8Error) -> (e)
            cause(err)
            description(err.description())
        }

        /// Ошибка конвертация числового поля.
        ParseIntError(err: ::std::num::ParseIntError) {
            from(e: ::std::num::ParseIntError) -> (e)
            cause(err)
            description(err.description())
        }

        /// Ошибка парсинга XML-словаря.
        /// Словарь не соответствует заданной структуре.
        ParsingError(err: String) {
            description("XML dictionary parsing error")
            display("XML dictionary parsing error: {}", err)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParsingState {
    Start,
    Dictionary,
    Grammemes,
    Grammeme,
    GrammemeName,
    GrammemeAlias,
    GrammemeDescription,
    Restrictions,
    Restriction,
    RestrictionLeft,
    RestrictionRight,
    Lemmata,
    Lemma,
    LemmaL,
    LemmaF,
    LinkTypes,
    LinkType,
    Links,
    End,
}

fn string_from_bytes(b: &[u8]) -> Result<String, std::str::Utf8Error> {
    let s = try!(std::str::from_utf8(b));
    Ok(s.to_owned())
}

fn integer_from_bytes<N>(b: &[u8]) -> Result<N, DictError>
    where N: std::str::FromStr<Err=std::num::ParseIntError>
{
    let s = try!(std::str::from_utf8(b));
    Ok(try!(s.parse()))
}

fn get_restriction_scope(el: &quick_xml::Element) -> Result<RestrictionScope, DictError> {
    for attr in el.attributes() {
        let (name, value) = try!(attr);
        match name {
            b"type" => {
                match value {
                    b"lemma" => {
                        return Ok(RestrictionScope::Lemma);
                    },
                    b"form" => {
                        return Ok(RestrictionScope::Form);
                    },
                    _ => {
                        let s = try!(std::str::from_utf8(value));
                        let errmsg = format!("invalid restriction scope: '{}'", s);
                        return Err(DictError::ParsingError(errmsg));
                    },
                }
            },
            _ => (),
        }
    }
    return Err(DictError::ParsingError("restriction scope isn't found".to_owned()));
}

fn get_grammeme(map: &std::collections::HashMap<String, std::rc::Rc<Grammeme>>, name: &[u8]) -> Result<std::rc::Rc<Grammeme>, DictError> {
    let s = try!(string_from_bytes(name));
    match map.get(&s) {
        Some(grm) => {
            Ok(grm.clone())
        },
        None => {
            Err(DictError::ParsingError(format!("invalid grammeme name: '{}'", s)))
        },
    }
}

fn get_lemma(map: &std::collections::HashMap<usize, std::rc::Rc<Lemma>>, id_str: &[u8]) -> Result<std::rc::Rc<Lemma>, DictError> {
    let id = try!(integer_from_bytes(id_str));
    match map.get(&id) {
        Some(lmt) => {
            Ok(lmt.clone())
        },
        None => {
            Err(DictError::ParsingError(format!("invalid lemma id: '{}'", id)))
        },
    }
}

impl Dict {
    /// читает словарь из XML-файла.
    pub fn read_from_xml<R: std::io::Read>(r: R) -> Result<Dict, DictError> {
        use std::io::BufReader;
        use std::rc::Rc;
        use std::str;
        use std::collections::HashMap;

        use quick_xml::{XmlReader, Event};

        let mut state = ParsingState::Start;

        let mut dict = Dict::default();

        let mut current_grammeme = Grammeme::default();
        let mut current_restriction = Restriction::default();
        let mut current_lemma = Lemma::default();
        let mut current_form = Form::default();
        let mut current_link_kind = LinkKind::default();

        let mut grammeme_by_name = HashMap::<String, Rc<Grammeme>>::new();
        let mut lemma_by_id = HashMap::<usize, Rc<Lemma>>::new();

        let reader = XmlReader::from_reader(BufReader::new(r));

        for ev in reader {
            match ev {
                Ok(Event::Start(ref el)) => {
                    match el.name() {
                        b"dictionary" if state == ParsingState::Start => {
                            state = ParsingState::Dictionary;
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"version" => {
                                        dict.version = try!(string_from_bytes(value));
                                    },
                                    b"revision" => {
                                        dict.revision = try!(integer_from_bytes(value))
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"grammemes" if state == ParsingState::Dictionary => {
                            state = ParsingState::Grammemes;
                            dict.grammemes.clear();
                            grammeme_by_name.clear();
                        },
                        b"grammeme" if state == ParsingState::Grammemes => {
                            state = ParsingState::Grammeme;
                            current_grammeme = Grammeme::default();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"parent" => {
                                        if value.is_empty() {
                                            current_grammeme.parent = None;
                                        } else {
                                            current_grammeme.parent = Some(try!(string_from_bytes(value)));
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"name" if state == ParsingState::Grammeme => {
                            state = ParsingState::GrammemeName;
                        },
                        b"alias" if state == ParsingState::Grammeme => {
                            state = ParsingState::GrammemeAlias;
                        },
                        b"description" if state == ParsingState::Grammeme => {
                            state = ParsingState::GrammemeDescription;
                        },
                        b"restrictions" if state == ParsingState::Dictionary => {
                            state = ParsingState::Restrictions;
                            dict.restrictions.clear();
                        },
                        b"restr" if state == ParsingState::Restrictions => {
                            state = ParsingState::Restriction;
                            current_restriction = Restriction::default();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"type" => {
                                        match value {
                                            b"maybe" => {
                                                current_restriction.kind = RestrictionKind::Maybe;
                                            },
                                            b"obligatory" => {
                                                current_restriction.kind = RestrictionKind::Obligatory;
                                            },
                                            b"forbidden" => {
                                                current_restriction.kind = RestrictionKind::Forbidden;
                                            },
                                            _ => {
                                                let s = try!(str::from_utf8(value));
                                                let errmsg = format!("invalid restriction kind: '{}'", s);
                                                return Err(DictError::ParsingError(errmsg));
                                            }
                                        }
                                    },
                                    b"auto" => {
                                        current_restriction.auto = try!(integer_from_bytes(value));
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"left" if state == ParsingState::Restriction => {
                            state = ParsingState::RestrictionLeft;
                            current_restriction.left_scope = try!(get_restriction_scope(el));
                        },
                        b"right" if state == ParsingState::Restriction => {
                            state = ParsingState::RestrictionRight;
                            current_restriction.right_scope = try!(get_restriction_scope(el));
                        },
                        b"lemmata" if state == ParsingState::Dictionary => {
                            state = ParsingState::Lemmata;
                            dict.lemmata.clear();
                        },
                        b"lemma" if state == ParsingState::Lemmata => {
                            state = ParsingState::Lemma;
                            current_lemma = Lemma::default();
                            current_lemma.forms.clear();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"id" => {
                                        current_lemma.id = try!(integer_from_bytes(value))
                                    },
                                    b"rev" => {
                                        current_lemma.revision = try!(integer_from_bytes(value))
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"l" if state == ParsingState::Lemma => {
                            state = ParsingState::LemmaL;
                            current_lemma.grammemes.clear();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"t" => {
                                        current_lemma.word = try!(string_from_bytes(value));
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"f" if state == ParsingState::Lemma => {
                            state = ParsingState::LemmaF;
                            current_form = Form::default();
                            current_form.grammemes.clear();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"t" => {
                                        current_form.word = try!(string_from_bytes(value));
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"g" if state == ParsingState::LemmaL => {
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"v" => {
                                        let grammeme = try!(get_grammeme(&grammeme_by_name, value));
                                        current_lemma.grammemes.push(grammeme);
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"g" if state == ParsingState::LemmaF => {
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"v" => {
                                        let grammeme = try!(get_grammeme(&grammeme_by_name, value));
                                        current_form.grammemes.push(grammeme);
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"link_types" if state == ParsingState::Dictionary => {
                            state = ParsingState::LinkTypes;
                            dict.link_kinds.clear();
                        },
                        b"type" if state == ParsingState::LinkTypes => {
                            state = ParsingState::LinkType;
                            current_link_kind = LinkKind::default();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"id" => {
                                        current_link_kind.id = try!(integer_from_bytes(value));
                                    },
                                    _ => (),
                                }
                            }
                        },
                        b"links" if state == ParsingState::Dictionary => {
                            state = ParsingState::Links;
                            dict.links.clear();
                        },
                        b"link" if state == ParsingState::Links => {
                            let mut current_link = Link::default();
                            for attr in el.attributes() {
                                let (name, value) = try!(attr);
                                match name {
                                    b"id" => {
                                        current_link.id = try!(integer_from_bytes(value));
                                    },
                                    b"from" => {
                                        let lemma = try!(get_lemma(&lemma_by_id, value));
                                        current_link.from = lemma;
                                    },
                                    b"to" => {
                                        let lemma = try!(get_lemma(&lemma_by_id, value));
                                        current_link.to = lemma;
                                    },
                                    b"type" => {
                                        let kind_id: usize = try!(integer_from_bytes(value));
                                        for lk in &dict.link_kinds {
                                            if kind_id == lk.id {
                                                current_link.kind = lk.clone();
                                            }
                                        }
                                    },
                                    _ => (),
                                }
                            }
                            dict.links.push(current_link.clone());
                        },
                        ref name => {
                            let s = try!(str::from_utf8(name));
                            return Err(DictError::ParsingError(format!("unexpected opening tag: '{}'", s)));
                        },
                    }
                },
                Ok(Event::Text(ref el)) => {
                    match state {
                        ParsingState::GrammemeName => {
                            current_grammeme.name = try!(string_from_bytes(el.content()));
                        },
                        ParsingState::GrammemeAlias => {
                            current_grammeme.alias = try!(string_from_bytes(el.content()));
                        },
                        ParsingState::GrammemeDescription => {
                            current_grammeme.description = try!(string_from_bytes(el.content()));
                        },
                        ParsingState::RestrictionLeft => {
                            if el.content().len() > 0 {
                                current_restriction.left_grammeme = Some(try!(get_grammeme(&grammeme_by_name, el.content())));
                            } else {
                                current_restriction.left_grammeme = None;
                            }
                        },
                        ParsingState::RestrictionRight => {
                            if el.content().len() > 0 {
                                current_restriction.right_grammeme = Some(try!(get_grammeme(&grammeme_by_name, el.content())));
                            } else {
                                current_restriction.right_grammeme = None;
                            }
                        },
                        ParsingState::LinkType => {
                            current_link_kind.name = try!(string_from_bytes(el.content()));
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref el)) => {
                    match el.name() {
                        b"dictionary" if state == ParsingState::Dictionary => {
                            state = ParsingState::End;
                        },
                        b"grammemes" if state == ParsingState::Grammemes => {
                            state = ParsingState::Dictionary;
                        },
                        b"grammeme" if state == ParsingState::Grammeme => {
                            state = ParsingState::Grammemes;
                            let last_grammeme = Rc::new(current_grammeme.clone());
                            dict.grammemes.push(last_grammeme.clone());
                            grammeme_by_name.insert(last_grammeme.name.clone(), last_grammeme.clone());
                        },
                        b"name" if state == ParsingState::GrammemeName => {
                            state = ParsingState::Grammeme;
                        },
                        b"alias" if state == ParsingState::GrammemeAlias => {
                            state = ParsingState::Grammeme;
                        },
                        b"description" if state == ParsingState::GrammemeDescription => {
                            state = ParsingState::Grammeme;
                        },
                        b"restrictions" if state == ParsingState::Restrictions => {
                            state = ParsingState::Dictionary;
                        },
                        b"restr" if state == ParsingState::Restriction => {
                            state = ParsingState::Restrictions;
                            dict.restrictions.push(current_restriction.clone());
                        },
                        b"left" if state == ParsingState::RestrictionLeft => {
                            state = ParsingState::Restriction;
                        },
                        b"right" if state == ParsingState::RestrictionRight => {
                            state = ParsingState::Restriction;
                        },
                        b"lemmata" if state == ParsingState::Lemmata => {
                            state = ParsingState::Dictionary;
                        },
                        b"lemma" if state == ParsingState::Lemma => {
                            state = ParsingState::Lemmata;
                            let last_lemma = Rc::new(current_lemma.clone());
                            dict.lemmata.push(last_lemma.clone());
                            lemma_by_id.insert(last_lemma.id, last_lemma.clone());
                        },
                        b"l" if state == ParsingState::LemmaL => {
                            state = ParsingState::Lemma;
                        },
                        b"f" if state == ParsingState::LemmaF => {
                            state = ParsingState::Lemma;
                            current_lemma.forms.push(current_form.clone());
                        },
                        b"g" if state == ParsingState::LemmaL || state == ParsingState::LemmaF => {},
                        b"link_types" if state == ParsingState::LinkTypes => {
                            state = ParsingState::Dictionary;
                        },
                        b"type" if state == ParsingState::LinkType => {
                            state = ParsingState::LinkTypes;
                            dict.link_kinds.push(Rc::new(current_link_kind.clone()));
                        },
                        b"links" if state == ParsingState::Links => {
                            state = ParsingState::Dictionary;
                        },
                        b"link" if state == ParsingState::Links => {},
                        ref name => {
                            let s = try!(str::from_utf8(name));
                            return Err(DictError::ParsingError(format!("unexpected closing tag: '{}'", s)));
                        },
                    }
                },
                Err((e, pos)) => return Result::Err(DictError::from((e, pos))),
                _ => (),
            }
        };

        if state != ParsingState::End {
            Err(DictError::ParsingError(format!("invalid state after parsing: {:?}", state)))
        } else {
            Ok(dict)
        }
    }
}
