use alloc::string::String;

use crate::{properties::{Color, Properties}, TileId};

/// Stores the data of the Wang color.
#[derive(Debug, PartialEq, Clone)]
pub struct WangColor {
    /// The name of this color.
    pub name: String,
    #[allow(missing_docs)]
    pub color: Color,
    /// The tile ID of the tile representing this color.
    pub tile: Option<TileId>,
    /// The relative probability that this color is chosen over others in case of multiple options. (defaults to 0)
    pub probability: f32,
    /// The custom properties of this color.
    pub properties: Properties,
}
