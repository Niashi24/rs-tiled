use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::iter::FromIterator;
use hashbrown::HashMap;
use no_std_io2::io::Read;

use xml::{reader::XmlEvent, EventReader};
use crate::{Error, Map, ResourceCache, ResourcePath, ResourceReader, Result};

pub fn parse_map(
    path: &ResourcePath,
    reader: &mut impl ResourceReader,
    cache: &mut impl ResourceCache,
) -> Result<Box<Map>> {
    // let mut parser = 
    //     // Box::new();
    let mut parser = EventReader::new(
        reader
            .read_from(path)
            .map_err(|err| Error::ResourceLoadingError {
                path: path.to_owned(),
                err: Box::new(err),
            })?,
    );
    loop {
        let next = parser.next().map_err(Error::XmlDecodingError)?;
        match next {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                if name.local_name == "map" {
                    return Map::parse_xml(
                        &mut Box::new(parser.into_iter()),
                        attributes,
                        path,
                        reader,
                        cache,
                    );
                }
            }
            XmlEvent::EndDocument => {
                return Err(Error::PrematureEnd(
                    "Document ended before map was parsed".to_string(),
                ))
            }
            _ => {}
        }
    }
}
