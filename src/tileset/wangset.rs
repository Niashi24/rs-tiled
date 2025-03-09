use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::{properties::Properties, TileId};

mod wang_color;
pub use wang_color::*;
mod wang_tile;
pub use wang_tile::*;

/// Wang set's terrain brush connection type.
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum WangSetType {
    Corner,
    Edge,
    Mixed,
}

impl Default for WangSetType {
    fn default() -> Self {
        WangSetType::Mixed
    }
}

/// Raw data belonging to a WangSet.
#[derive(Debug, PartialEq, Clone)]
pub struct WangSet {
    /// The name of the Wang set.
    pub name: String,
    /// Type of Wang set.
    pub wang_set_type: WangSetType,
    /// The tile ID of the tile representing this Wang set.
    pub tile: Option<TileId>,
    /// The colors color that can be used to define the corner and/or edge of each Wang tile.
    pub wang_colors: Vec<WangColor>,
    ///  All the Wang tiles present in this Wang set, indexed by their local IDs.
    pub wang_tiles: HashMap<TileId, WangTile>,
    /// The custom properties of this Wang set.
    pub properties: Properties,
}
