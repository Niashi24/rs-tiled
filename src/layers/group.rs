use alloc::vec::Vec;

use crate::{layers::LayerData, util::*, Layer};

/// The raw data of a [`GroupLayer`]. Does not include a reference to its parent [`Map`](crate::Map).
#[derive(Debug, PartialEq, Clone)]
pub struct GroupLayerData {
    layers: Vec<LayerData>,
}

map_wrapper!(
    #[doc = "A group layer, used to organize the layers of the map in a hierarchy."]
    #[doc = "\nAlso see the [TMX docs](https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#group)."]
    #[doc = "## Note"]
    #[doc = "In Tiled, the properties of the group layer recursively affect child layers.
    Implementing this behavior is left up to the user of this library."]
    GroupLayer => GroupLayerData
);

impl<'map> GroupLayer<'map> {
    /// Returns an iterator over the layers present in this group in display order.
    /// ## Example
    /// ```
    /// use tiled::Layer;
    /// # use tiled::Loader;
    ///
    /// # fn main() {
    /// # let map = Loader::new()
    /// #     .load_tmx_map("assets/tiled_group_layers.tmx")
    /// #     .unwrap();
    /// #
    /// let nested_layers: Vec<Layer> = map
    ///     .layers()
    ///     .filter_map(|layer| match layer.layer_type() {
    ///         tiled::LayerType::Group(layer) => Some(layer),
    ///         _ => None,
    ///     })
    ///     .flat_map(|layer| layer.layers())
    ///     .collect();
    ///
    /// dbg!(nested_layers);
    /// # }
    /// ```
    pub fn layers(&self) -> impl ExactSizeIterator<Item = Layer<'map>> + 'map {
        let map: &'map crate::Map = self.map;
        self.data
            .layers
            .iter()
            .map(move |layer| Layer::new(map, layer))
    }
    /// Gets a specific layer from the group by index.
    pub fn get_layer(&self, index: usize) -> Option<Layer<'map>> {
        self.data
            .layers
            .get(index)
            .map(|data| Layer::new(self.map, data))
    }
}
