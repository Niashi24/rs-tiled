use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use portable_atomic_util::Arc;
use quick_xml::events::Event;

use crate::{
    parent,
    parse::xml::{Parser, Reader},
    util::*,
    EmbeddedParseResultType, Error, MapTilesetGid, ObjectData, ResourceCache, ResourcePath, ResourcePathBuf, Result, Tileset
};
use crate::parse::xml::ReadFrom;

/// A template, consisting of an object and a tileset
///
/// Templates define a tileset and object data to use for an object that can be shared between multiple objects and
/// maps.
#[derive(Clone, Debug)]
pub struct Template {
    /// The path first used in a [`ResourceReader`] to load this template.
    pub source: ResourcePathBuf,
    /// The tileset this template contains a reference to
    pub tileset: Option<Arc<Tileset>>,
    /// The object data for this template
    pub object: ObjectData,
}

impl Template {
    pub(crate) fn parse_template(
        path: &ResourcePath,
        read_from: &mut impl ReadFrom,
        cache: &mut impl ResourceCache,
    ) -> Result<Arc<Template>> {
        // Open the template file
        let mut file = read_from
            .read_from(path)
            
            .map_err(|err| Error::ResourceLoadingError {
                path: path.to_owned(),
                err: Box::new(err),
            })?;

        let mut buffer = Vec::new();
        loop {
            let next = file
                .read_event_into(&mut buffer)
                
                .map_err(Error::XmlDecodingError)?;
            match next {
                Event::Start(start) if start.local_name().into_inner() == b"template" => {
                    let template = Self::parse_external_template(
                        &mut Parser::with_reader(file),
                        path,
                        read_from,
                        cache,
                    )?;
                    return Ok(template);
                }
                Event::Eof => {
                    return Err(Error::PrematureEnd(
                        "Template Document ended before template element was parsed".to_string(),
                    ))
                }
                _ => {}
            }
        }
    }

    fn parse_external_template<R: Reader>(
        parser: &mut Parser<R>,
        template_path: &ResourcePath,
        read_from: &mut impl ReadFrom,
        cache: &mut impl ResourceCache,
    ) -> Result<Arc<Template>> {
        let mut object = Option::None;
        let mut tileset = None;
        let mut tileset_gid: Vec<MapTilesetGid> = vec![];

        let mut buffer = Vec::new();
        parse_tag!(parser => &mut buffer, "template", {
            "object" => for attrs {
                object = Some(ObjectData::new(parser, attrs, Some(&tileset_gid), tileset.clone(), parent(template_path).ok_or(Error::PathIsNotFile)?, read_from, cache)?);
                Ok(())
            },
            "tileset" => for attrs {
                let res = Tileset::parse_xml_in_map(parser, &attrs, template_path, read_from, cache)?;
                match res.result_type {
                    EmbeddedParseResultType::ExternalReference { tileset_path } => {
                        tileset = Some(if let Some(ts) = cache.get_tileset(&tileset_path) {
                            ts
                        } else {
                            let tileset = Arc::new(crate::parse::xml::parse_tileset(&tileset_path, read_from, cache)?);
                            cache.insert_tileset(tileset_path.clone(), tileset.clone());
                            tileset
                        });
                    }
                    EmbeddedParseResultType::Embedded { tileset: embedded_tileset } => {
                        tileset = Some(Arc::new(embedded_tileset));
                    },
                };
                tileset_gid.push(MapTilesetGid {
                    tileset: tileset.clone().unwrap(),
                    first_gid: res.first_gid,
                });
                Ok(())
            },
        });

        let object = object.ok_or(Error::TemplateHasNoObject)?;

        Ok(Arc::new(Template {
            source: template_path.to_owned(),
            tileset,
            object,
        }))
    }
}
