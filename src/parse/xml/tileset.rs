use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use itertools::Itertools;
use quick_xml::events::Event;

use crate::{Error, ResourceCache, ResourcePath, Result, Tileset};

use super::{Parser, ReadFrom, Reader};

pub async fn parse_tileset(
    path: &ResourcePath,
    read_from: &mut impl ReadFrom,
    cache: &mut impl ResourceCache,
) -> Result<Tileset> {
    let mut reader =
        Box::pin(read_from
            .read_from(path))
            .await
            .map_err(|err| Error::ResourceLoadingError {
                path: path.to_owned(),
                err: Box::new(err),
            })?;
    let mut buffer = Vec::new();
    loop {
        match Box::pin(reader
            .read_event_into(&mut buffer))
            .await
            .map_err(Error::XmlDecodingError)?
        {
            Event::Start(start) if start.local_name().into_inner() == b"tileset" => {
                let attributes: Vec<_> = start
                    .attributes()
                    .try_collect()
                    .map_err(|err| Error::XmlDecodingError(err.into()))?;

                return Box::pin(Tileset::parse_external_tileset(
                    &mut Parser::with_reader(reader),
                    &attributes,
                    path,
                    read_from,
                    cache,
                ))
                    .await;
            }
            Event::Eof => {
                return Err(Error::PrematureEnd(
                    "Tileset Document ended before map was parsed".to_string(),
                ))
            }
            
            _ => {}
        }
    }
}
