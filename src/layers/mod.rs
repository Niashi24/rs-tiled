use alloc::string::String;

use crate::{error::Result, parent, properties::Properties, util::*, Color, Map, MapTilesetGid, ResourceCache, ResourcePath, Tileset};

mod image;
pub use image::*;
mod object;
pub use object::*;
mod tile;
pub use tile::*;
mod group;
pub use group::*;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum LayerDataType {
    Tiles(TileLayerData),
    Objects(ObjectLayerData),
    Image(ImageLayerData),
    Group(GroupLayerData),
}

#[derive(Clone, Copy)]
pub(crate) enum LayerTag {
    Tiles,
    Objects,
    Image,
    Group,
}

/// The raw data of a [`Layer`]. Does not include a reference to its parent [`Map`](crate::Map).
#[derive(Clone, PartialEq, Debug)]
pub struct LayerData {
    /// The layer's name, set arbitrarily by the user.
    pub name: String,
    id: u32,
    /// Whether this layer should be visible or not.
    pub visible: bool,
    /// The layer's x offset (in pixels).
    pub offset_x: f32,
    /// The layer's y offset (in pixels).
    pub offset_y: f32,
    /// The layer's x parallax factor.
    pub parallax_x: f32,
    /// The layer's y parallax factor.
    pub parallax_y: f32,
    /// The layer's opacity.
    pub opacity: f32,
    /// The layer's tint color.
    pub tint_color: Option<Color>,
    /// The layer's custom properties, as arbitrarily set by the user.
    pub properties: Properties,
    /// The layer's type, which is arbitrarily setby the user.
    pub user_type: Option<String>,
    layer_type: LayerDataType,
}

impl LayerData {
    /// Get the layer's id. Unique within the parent map. Valid only if greater than 0. Defaults to
    /// 0 if the layer was loaded from a file that didn't have the attribute present.
    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

}

map_wrapper!(
    #[doc = "A generic map layer, accessed via [`Map::layers()`]."]
    Layer => LayerData
);

impl<'map> Layer<'map> {
    /// Get the layer's type.
    #[inline]
    pub fn layer_type(&self) -> LayerType<'map> {
        LayerType::new(self.map, &self.data.layer_type)
    }

    /// Convenience method to return this layer as a tile layer, only if it is one.
    ///
    /// Identical to:
    /// ```ignore
    /// match layer.layer_type() {
    ///     LayerType::Tiles(x) => Some(x),
    ///     _ => None,
    /// }
    /// ```
    #[inline]
    pub fn as_tile_layer(self) -> Option<TileLayer<'map>> {
        match self.layer_type() {
            LayerType::Tiles(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method to return this layer as an object group, only if it is one.
    ///
    /// Identical to:
    /// ```ignore
    /// match layer.layer_type() {
    ///     LayerType::Objects(x) => Some(x),
    ///     _ => None,
    /// }
    /// ```
    #[inline]
    pub fn as_object_layer(self) -> Option<ObjectLayer<'map>> {
        match self.layer_type() {
            LayerType::Objects(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method to return this layer as an image layer, only if it is one.
    ///
    /// Identical to:
    /// ```ignore
    /// match layer.layer_type() {
    ///     LayerType::Image(x) => Some(x),
    ///     _ => None,
    /// }
    /// ```
    #[inline]
    pub fn as_image_layer(self) -> Option<ImageLayer<'map>> {
        match self.layer_type() {
            LayerType::Image(x) => Some(x),
            _ => None,
        }
    }

    /// Convenience method to return this layer as a group layer, only if it is one.
    ///
    /// Identical to:
    /// ```ignore
    /// match layer.layer_type() {
    ///     LayerType::Group(x) => Some(x),
    ///     _ => None,
    /// }
    /// ```
    #[inline]
    pub fn as_group_layer(self) -> Option<GroupLayer<'map>> {
        match self.layer_type() {
            LayerType::Group(x) => Some(x),
            _ => None,
        }
    }
}

/// Represents some kind of map layer.
#[derive(Debug)]
pub enum LayerType<'map> {
    /// A tile layer; Also see [`TileLayer`].
    Tiles(TileLayer<'map>),
    /// An object layer (also called object group); Also see [`ObjectLayer`].
    Objects(ObjectLayer<'map>),
    /// An image layer; Also see [`ImageLayer`].
    Image(ImageLayer<'map>),
    /// A group layer; Also see [`GroupLayer`].
    Group(GroupLayer<'map>),
}

impl<'map> LayerType<'map> {
    fn new(map: &'map Map, data: &'map LayerDataType) -> Self {
        match data {
            LayerDataType::Tiles(data) => Self::Tiles(TileLayer::new(map, data)),
            LayerDataType::Objects(data) => Self::Objects(ObjectLayer::new(map, data)),
            LayerDataType::Image(data) => Self::Image(ImageLayer::new(map, data)),
            LayerDataType::Group(data) => Self::Group(GroupLayer::new(map, data)),
        }
    }
}
