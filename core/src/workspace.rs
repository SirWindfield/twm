//!

use crate::display::Display;
use crate::layout::{Layout, LayoutUpdateInfo};
use crate::tile::{Tile, TileId};
use crate::layout::sided_layout::SidedLayout;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// A workspace id.
pub type WorkspaceId = u32;

/// A workspace keeps track of all tiles and the currently focused tile inside of it.
///
/// Workspaces have one active layout associated with it that is responsible for laying the tiles out.
#[derive(Clone, Debug, Derivative, Deserialize, Serialize)]
pub struct Workspace {
    /// The unique id.
    pub id: WorkspaceId,
    /// The display the workspace is in.
    pub display: Display,
    /// The tiles inside this workspace.
    tiles: Vec<Tile>,
    /// The currently active layout.
    pub layout: Box<dyn Layout>,
    /// The id of the focused tile. If `None`, no tile is focused.
    pub focused_tile_id: Option<TileId>,
}

impl Default for Workspace {
    /// Returns a default instance.
    ///
    /// The default value for the `layout` field is an instance of `SidedLayout`.
    fn default() -> Self {
        Self {
            id: 0,
            display: Display::default(),
            tiles: Vec::default(),
            layout: Box::new(SidedLayout::new()),
            focused_tile_id: None,
        }
    }
}

impl Workspace {
    /// Creates a new default `Workspace`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a workspace.
    ///
    /// # Arguments
    ///
    /// - `id`: The id of the workspace.
    /// - `display`: The display that the workspace resides in.
    pub fn with(id: WorkspaceId, display: Display) -> Self {
        Self {
            id,
            display,
            ..Self::default()
        }
    }

    /// Lays out all the tiles inside the workspace.
    pub fn layout(&mut self) {
        let mut update_info = LayoutUpdateInfo {
            tiles: &mut self.tiles,
            workspace_bbox: self.display.bbox,
        };
        self.layout.layout(&mut update_info);
    }

    /// Adds a new tile to the workspace and focuses it.
    ///
    /// # Note
    ///
    /// This method does __NOT__ check if a `Tile` with the same id is already inside the workspace. ID management is up to the caller.
    pub fn add_tile(&mut self, tile: Tile) {
        self.focused_tile_id = Some(tile.id);
        self.tiles.push(tile);
    }

    /// Removes a tile from the workspace.
    ///
    /// If the `Tile` was focused, the `focused_tile_id` is set to `None`.
    pub fn remove_tile(&mut self, tile: &Tile) {
        self.remove_tile_by_id(tile.id);
    }

    /// Removes a tile with a given id from the workspace.
    ///
    /// If the `Tile` was focused, the `focused_tile_id` is set to `None`.
    pub fn remove_tile_by_id(&mut self, tile_id: TileId) {
        if self.focused_tile_id == Some(tile_id) {
            self.focused_tile_id = None;
        }
        let (index, _) = self
            .iter()
            .enumerate()
            .find(|(_index, tile)| tile.id == tile_id)
            .expect("Failed to find tile with right id");
        self.tiles.remove(index);
    }

    /// Returns a reference of all tiles inside the workspace.
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    /// Returns a mutable reference of all tiles inside the workspace.
    pub fn tiles_mut(&mut self) -> &Vec<Tile> {
        &self.tiles
    }

    /// Returns a reference of the focused tile.
    ///
    /// # Returns
    ///
    /// `Some(&Tile)` if a tile is focused, `None` otherwise.
    pub fn focused_tile(&self) -> Option<&Tile> {
        let focused_tile_id = self.focused_tile_id;
        self.iter().find(|t| Some(t.id) == focused_tile_id)
    }

    /// Returns a mutable reference of the focused tile.
    ///
    /// # Returns
    ///
    /// `Some(&mut Tile)` if a tile is focused, `None` otherwise.
    pub fn focused_tile_mut(&mut self) -> Option<&mut Tile> {
        let focused_tile_id = self.focused_tile_id;
        self.iter_mut().find(|t| Some(t.id) == focused_tile_id)
    }

    /// Returns a reference to a tile given by its id.
    ///
    /// # Arguments
    ///
    /// - `id`: The id of the tile.
    ///
    /// # Returns
    ///
    /// `Some(&Tile)` if the tile exists, `None` otherwise.
    pub fn tile_by_id(&self, id: TileId) -> Option<&Tile> {
        self.iter().find(|t| id == t.id)
    }

    /// Returns a mutable reference to a tile given by its id.
    ///
    /// # Arguments
    ///
    /// - `id`: The id of the tile.
    ///
    /// # Returns
    ///
    /// `Some(&mut Tile)` if the tile exists, `None` otherwise.
    pub fn tile_by_id_mut(&mut self, id: TileId) -> Option<&mut Tile> {
        self.iter_mut().find(|t| t.id == id)
    }

    /// Returns an `Iterator` over all tiles inside this workspace.
    pub fn iter(&self) -> std::slice::Iter<Tile> {
        self.tiles.iter()
    }

    /// Returns a mutable `Iterator` over all tiles inside this workspace.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Tile> {
        self.tiles.iter_mut()
    }
}

impl IntoIterator for Workspace {
    type Item = Tile;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_iter()
    }
}
