use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::iter::FromIterator;
use hashbrown::HashMap;
use no_std_io2::io::Read;
use quick_xml::events::Event;
// use xml::{reader::XmlEvent, EventReader};
use crate::{Error, Map, ResourceCache, ResourcePath, ResourceReader, Result};
use crate::parse::xml::{Parser, ReadFrom, Reader};
use itertools::Itertools;

pub fn print_size<T: Sized>(name: &str, _: &T) {
    playdate::println!("{name}: {}", core::mem::size_of::<T>());
}

macro_rules! print_size {
    ($var:expr) => {
        crate::parse::xml::map::print_size(stringify!($var), $var)
    };
}

pub fn parse_map(
    path: &ResourcePath,
    read_from: &mut impl ReadFrom,
    cache: &mut impl ResourceCache,
) -> Result<Map> {
    let mut reader =
        read_from
            .read_from(path)
            .map_err(|err| Error::ResourceLoadingError {
                path: path.to_owned(),
                err: Box::new(err),
            })?;
    print_size!(&reader);
    let mut buffer = Vec::new();
    loop {
        let next = reader
        .read_event_into(&mut buffer)
        .map_err(Error::XmlDecodingError)?;
        print_size!(&next);
        match next
        {
            Event::Start(start) if start.local_name().into_inner() == b"map" => {
                let attributes = start
                    .attributes()
                    .try_collect()
                    .map_err(|err| Error::XmlDecodingError(err.into()))?;
                let mut parser = Parser::with_reader(reader);
                print_size!(&parser);
                return Map::parse_xml(&mut parser, attributes, path, read_from, cache);
            }
            Event::Eof => {
                return Err(Error::PrematureEnd(
                    "Document ended before map was parsed".to_string(),
                ))
            }
            _ => {}
        }
    }
}
