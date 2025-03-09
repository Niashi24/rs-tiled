use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;
use base64::Engine;
use quick_xml::encoding::EncodingError;
use quick_xml::events::Event;

use crate::{
    parse::xml::{Parser, Reader},
    CsvDecodingError, Error, LayerTileData, MapTilesetGid, Result
};

pub(crate) fn parse_data_line<R: Reader>(
    encoding: Option<&str>,
    compression: Option<&str>,
    parser: &mut Parser<R>,
    tilesets: &[MapTilesetGid],
) -> Result<Vec<Option<LayerTileData>>> {
    match (encoding, compression) {
        (Some("csv"), None) => decode_csv(parser, tilesets),

        // (Some("base64"), None) => parse_base64(parser).map(|v| convert_to_tiles(&v, tilesets)),
        // (Some("base64"), Some("zlib")) => parse_base64(parser)
        //     .and_then(|data| process_decoder(Ok(flate2::bufread::ZlibDecoder::new(&data[..]))))
        //     .map(|v| convert_to_tiles(&v, tilesets)),
        // (Some("base64"), Some("gzip")) => parse_base64(parser)
        //     .and_then(|data| process_decoder(Ok(flate2::bufread::GzDecoder::new(&data[..]))))
        //     .map(|v| convert_to_tiles(&v, tilesets)),
        // #[cfg(feature = "zstd")]
        // (Some("base64"), Some("zstd")) => parse_base64(parser)
        //     .and_then(|data| process_decoder(zstd::stream::read::Decoder::with_buffer(&data[..])))
        //     .map(|v| convert_to_tiles(&v, tilesets)),

        _ => Err(Error::InvalidEncodingFormat {
            encoding: encoding.map(ToOwned::to_owned),
            compression: compression.map(ToOwned::to_owned),
        }),
    }
}

// fn parse_base64(parser: &mut impl Iterator<Item = XmlEventResult>) -> Result<Vec<u8>> {
//     for next in parser {
//         match next.map_err(Error::XmlDecodingError)? {
//             XmlEvent::Characters(s) => {
//                 return base64::engine::GeneralPurpose::new(
//                     &base64::alphabet::STANDARD,
//                     base64::engine::general_purpose::PAD,
//                 )
//                 .decode(s.trim().as_bytes())
//                 .map_err(Error::Base64DecodingError);
//             }
//             XmlEvent::EndElement { name, .. } if name.local_name == "data" => {
//                 return Ok(Vec::new());
//             }
//             _ => {}
//         }
//     }
//     Err(Error::PrematureEnd("Ran out of XML data".to_owned()))
// }

// fn process_decoder(decoder: std::io::Result<impl Read>) -> Result<Vec<u8>> {
//     decoder
//         .and_then(|mut decoder| {
//             let mut data = Vec::new();
//             decoder.read_to_end(&mut data)?;
//             Ok(data)
//         })
//         .map_err(Error::DecompressingError)
// }

fn decode_csv<R: Reader>(
    parser: &mut Parser<R>,
    tilesets: &[MapTilesetGid],
) -> Result<Vec<Option<LayerTileData>>> {
    loop {
        let next = parser.read_event().map_err(Error::XmlDecodingError)?;
        match next {
            Event::Text(text) => {
                let text = core::str::from_utf8(&text)
                    .map_err(|err| Error::XmlDecodingError(quick_xml::Error::Encoding(EncodingError::Utf8(err))))?;
                let mut tiles = Vec::new();
                for v in text.split(',') {
                    match v.trim().parse() {
                        Ok(bits) => tiles.push(LayerTileData::from_bits(bits, tilesets)),
                        Err(e) => {
                            return Err(Error::CsvDecodingError(
                                CsvDecodingError::TileDataParseError(e),
                            ))
                        }
                    }
                }
                return Ok(tiles);
            }
            Event::End(end) if end.local_name().into_inner() == b"data" => return Ok(Vec::new()),
            Event::Eof => return Err(Error::PrematureEnd("Ran out of XML data".to_owned())),
            _ => {}
        }
    }
}

fn convert_to_tiles(data: &[u8], tilesets: &[MapTilesetGid]) -> Vec<Option<LayerTileData>> {
    data.chunks_exact(4)
        .map(|chunk| {
            let bits = u32::from_le_bytes(chunk.try_into().unwrap());
            LayerTileData::from_bits(bits, tilesets)
        })
        .collect()
}
