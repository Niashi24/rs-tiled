use alloc::string::String;
use alloc::vec::Vec;
use portable_atomic_util::Arc;

use crate::{properties::Properties, util::map_wrapper, Color, Gid, MapTilesetGid, ResourceCache, Tile, TileId, Tileset};

/// The location of the tileset this tile is in
///
/// Tilesets can be contained within either a map or a template.
#[derive(Clone, Debug, PartialEq)]
pub enum TilesetLocation {
    /// Index into the Map's tileset list, guaranteed to be a valid index of the map tileset container.
    Map(usize),
    /// Arc of the tileset itself if and only if this location is from a template.
    Template(Arc<Tileset>),
}

/// Stores the internal tile gid about a layer tile, along with how it is flipped.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectTileData {
    /// A valid TilesetLocation that points to a tileset that **may or may not contain** this tile.
    tileset_location: TilesetLocation,
    /// The local ID of the tile in the tileset it's in.
    id: TileId,
    /// Whether this tile is flipped on its Y axis (horizontally).
    pub flip_h: bool,
    /// Whether this tile is flipped on its X axis (vertically).
    pub flip_v: bool,
    /// Whether this tile is flipped diagonally.
    pub flip_d: bool,
}

impl ObjectTileData {
    /// Get the layer tile's local id within its parent tileset.
    #[inline]
    pub fn id(&self) -> TileId {
        self.id
    }

    /// Get a reference to the object tile data's tileset location, which points to a tileset that
    /// **may or may not contain** this tile.
    #[inline]
    pub fn tileset_location(&self) -> &TilesetLocation {
        &self.tileset_location
    }

    const FLIPPED_HORIZONTALLY_FLAG: u32 = 0x80000000;
    const FLIPPED_VERTICALLY_FLAG: u32 = 0x40000000;
    const FLIPPED_DIAGONALLY_FLAG: u32 = 0x20000000;
    const ALL_FLIP_FLAGS: u32 = Self::FLIPPED_HORIZONTALLY_FLAG
        | Self::FLIPPED_VERTICALLY_FLAG
        | Self::FLIPPED_DIAGONALLY_FLAG;

    /// Creates a new [`ObjectTileData`] from a [`Gid`] plus its flipping bits.
    pub(crate) fn from_bits(
        bits: u32,
        tilesets: &[MapTilesetGid],
        for_tileset: Option<Arc<Tileset>>,
    ) -> Option<Self> {
        let flags = bits & Self::ALL_FLIP_FLAGS;
        let gid = Gid(bits & !Self::ALL_FLIP_FLAGS);
        let flip_d = flags & Self::FLIPPED_DIAGONALLY_FLAG == Self::FLIPPED_DIAGONALLY_FLAG; // Swap x and y axis (anti-diagonally) [flips over y = -x line]
        let flip_h = flags & Self::FLIPPED_HORIZONTALLY_FLAG == Self::FLIPPED_HORIZONTALLY_FLAG; // Flip tile over y axis
        let flip_v = flags & Self::FLIPPED_VERTICALLY_FLAG == Self::FLIPPED_VERTICALLY_FLAG; // Flip tile over x axis

        if gid == Gid::EMPTY {
            None
        } else {
            let (tileset_location, id) = match for_tileset {
                Some(tileset) => (TilesetLocation::Template(tileset), gid.0 - 1),
                None => {
                    let (tileset_index, tileset) = crate::util::get_tileset_for_gid(tilesets, gid)?;
                    let id = gid.0 - tileset.first_gid.0;
                    (TilesetLocation::Map(tileset_index), id)
                }
            };

            Some(Self {
                tileset_location,
                id,
                flip_h,
                flip_v,
                flip_d,
            })
        }
    }
}

map_wrapper!(
    #[doc = "An instance of a [`Tile`] present in an [`Object`]."]
    ObjectTile => ObjectTileData
);

impl<'map> ObjectTile<'map> {
    /// Get a reference to the object tile's referenced tile, if it exists.
    #[inline]
    pub fn get_tile(&self) -> Option<Tile<'map>> {
        self.get_tileset().get_tile(self.data.id)
    }
    /// Get a reference to the object tile's referenced tileset.
    #[inline]
    pub fn get_tileset(&self) -> &'map Tileset {
        match &self.data.tileset_location {
            // SAFETY: `tileset_index` is guaranteed to be valid
            TilesetLocation::Map(n) => &self.map.tilesets()[*n],
            TilesetLocation::Template(t) => t,
        }
    }
}

/// A structure describing an [`Object`]'s shape.
///
/// Also see the [TMX docs](https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tmx-object).
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub enum ObjectShape {
    Rect {
        width: f32,
        height: f32,
    },
    Ellipse {
        width: f32,
        height: f32,
    },
    Polyline {
        points: Vec<(f32, f32)>,
    },
    Polygon {
        points: Vec<(f32, f32)>,
    },
    Point(f32, f32),
    Text {
        font_family: String,
        pixel_size: usize,
        wrap: bool,
        color: Color,
        bold: bool,
        italic: bool,
        underline: bool,
        strikeout: bool,
        kerning: bool,
        halign: HorizontalAlignment,
        valign: VerticalAlignment,
        /// The actual text content of this object.
        text: String,
        width: f32,
        height: f32,
    },
}

/// The horizontal alignment of an [`ObjectShape::Text`].
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[allow(missing_docs)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

/// The vertical alignment of an [`ObjectShape::Text`].
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[allow(missing_docs)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

/// Raw data belonging to an object. Used internally and for tile collisions.
///
/// Also see the [TMX docs](https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tmx-object).
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectData {
    id: u32,
    tile: Option<ObjectTileData>,
    /// The name of the object, which is arbitrary and set by the user.
    pub name: String,
    /// The type of the object, which is arbitrary and set by the user.
    pub user_type: String,
    /// The X coordinate of this object in pixels.
    pub x: f32,
    /// The Y coordinate of this object in pixels.
    pub y: f32,
    /// The clockwise rotation of this object around (x,y) in degrees.
    pub rotation: f32,
    /// Whether the object is shown or hidden.
    pub visible: bool,
    /// The object's shape.
    pub shape: ObjectShape,
    /// The object's custom properties as set by the user.
    pub properties: Properties,
}

impl ObjectData {
    /// ID of the object, which is unique per map since Tiled 0.11.
    ///
    /// On older versions this value is defaulted to 0.
    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the data of the tile that this object is referencing, if it exists.
    #[inline]
    pub fn tile_data(&self) -> Option<ObjectTileData> {
        self.tile.clone()
    }
}

map_wrapper!(
    #[doc = "Wrapper over an [`ObjectData`] that contains both a reference to the data as well as
    to the map it is contained in."]
    Object => ObjectData
);

impl<'map> Object<'map> {
    /// Returns the tile that the object is using as image, if any.
    pub fn get_tile(&self) -> Option<ObjectTile<'map>> {
        self.data
            .tile
            .as_ref()
            .map(|tile| ObjectTile::new(self.map, tile))
    }
}
