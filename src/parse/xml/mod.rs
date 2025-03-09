mod map;

use alloc::vec::Vec;
use no_std_io2::io::BufRead;
pub(crate) use map::*;
mod tileset;
pub use tileset::*;

pub(crate) use quick_xml::events::Event;
pub(crate) use quick_xml::Reader as RawReader;
pub(crate) use quick_xml::Result as ReadResult;
pub(crate) use tileset::*;

use crate::{ResourcePath};
use crate::ResourceReader;

/// An abstraction of [`RawReader`] that comes in two flavors: [`SyncEventReader`] and
/// [`AsyncEventReader`].
pub(crate) trait Reader {
    /// Delegates to either [`RawReader::read_event_into`] or [`RawReader::read_event_into_async`],
    /// depending on the implementor.
    fn read_event_into<'b>(&mut self, buf: &'b mut Vec<u8>) -> ReadResult<Event<'b>>;
}

/// A [`RawReader`] in 'sync' mode, i.e. that will delegate to [`RawReader::read_event_into`].
pub(crate) struct SyncReader<R>(pub(crate) RawReader<R>);

impl<R: BufRead> Reader for SyncReader<R> {
    /// Will immediately return the next event on the first poll.
    fn read_event_into<'b>(&mut self, buf: &'b mut Vec<u8>) -> ReadResult<Event<'b>> {
        self.0.read_event_into(buf)
    }
}

/// A [`RawReader`] in 'async' mode, i.e. that will delegate to [`RawReader::read_event_into_async`].
pub(crate) struct AsyncReader<R>(pub(crate) RawReader<R>);

/// An abstraction of the [`ResourceReader`] and [`AsyncResourceReader`] traits that comes in two
/// flavors: [`SyncReadFrom`] and [`AsyncReadFrom`].
pub(crate) trait ReadFrom {
    type Reader: Reader;
    type Error: core::error::Error + Send + Sync + 'static;
    fn read_from(&mut self, path: &ResourcePath) -> Result<Self::Reader, Self::Error>;
}

/// Wraps a [`ResourceReader`].
pub(crate) struct SyncReadFrom<'r, R>(pub(crate) &'r mut R);

impl<R: ResourceReader> ReadFrom for SyncReadFrom<'_, R> {
    type Reader = SyncReader<R::Resource>;
    type Error = R::Error;

    /// Returns on the first poll.
    fn read_from(&mut self, path: &ResourcePath) -> Result<Self::Reader, Self::Error> {
        let resource = self.0.read_from(path)?;
        Ok(SyncReader(RawReader::from_reader(resource)))
    }
}

/// Wraps an [`AsyncResourceReader`].
pub(crate) struct AsyncReadFrom<'r, R>(pub(crate) &'r mut R);
// 
// impl<R: AsyncResourceReader> ReadFrom for AsyncReadFrom<'_, R> {
//     type Reader = AsyncReader<R::Resource>;
//     type Error = R::Error;
// 
//     fn read_from(&mut self, path: &ResourcePath) -> Result<Self::Reader, Self::Error> {
//         let resource = self.0.read_from(path)?;
//         Ok(AsyncReader(RawReader::from_reader(resource)))
//     }
// }

/// A [`Reader`]-buffer pair.
pub(crate) struct Parser<R> {
    pub(crate) reader: R,
    pub(crate) buffer: Vec<u8>,
    pub(crate) last_event_was_empty: bool,
}

impl<R> Parser<R> {
    /// Creates a [`Parser`] with the specified [`Reader`] and an empty buffer.
    pub(crate) fn with_reader(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            last_event_was_empty: false,
        }
    }
}

impl<R: Reader> Parser<R> {
    pub(crate) fn read_event(&mut self) -> ReadResult<Event> {
        let event = self.reader.read_event_into(&mut self.buffer)?;
        self.last_event_was_empty = matches!(event, Event::Empty(_));
        Ok(event)
    }
    
    pub(crate) fn read_event_into<'a>(
        &mut self,
        buf: &'a mut Vec<u8>,
    ) -> ReadResult<Event<'a>> {
        let event = self.reader.read_event_into(buf)?;
        self.last_event_was_empty = matches!(event, Event::Empty(_));
        Ok(event)
    }
}
