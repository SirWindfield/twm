//! Tiles are used to move around and resize windows inside a workspace.

use crate::{bbox::BBox, window::Window};
use serde::{Deserialize, Serialize};

/// The unique id of a tile.
pub type TileId = u32;

/// A tile inside a workspace.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Tile {
    /// The unique id of the tile.
    pub id: TileId,
    /// The bounding box of the tile.
    pub bbox: BBox,
    /// The window this tile displays.
    pub window: Window,
}

impl Tile {
    /// Creates an empty tile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a tile.
    ///
    /// # Arguments
    ///
    /// - `id`: The id of the tile.
    /// - `bbox`: The bounding box of the tile.
    /// - `window`: The window that the tiles displays.
    ///
    /// # Returns
    ///
    /// A new tile.
    ///
    /// # Note
    ///
    /// `Tile`s do not keep track of already assigned IDs. It's up to the
    /// library user to take care of this.
    pub fn with(id: TileId, bbox: BBox, window: Window) -> Self {
        Self { id, bbox, window }
    }
}
