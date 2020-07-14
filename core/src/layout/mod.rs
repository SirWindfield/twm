//! A layout is responsible for laying out tiles inside a workspace.
//!
//! These is done by changing the bounding box of each tile according to the layout's implementation details.

use crate::bbox::BBox;
use crate::tile::Tile;
pub use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;

pub mod middle_layout;
pub mod sided_layout;

/// Metadata associated with a layout.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LayoutMeta {
    /// The name of the layout. This should be a human-readable string.
    pub name: String,
}

/// The information needed to update tiles inside a workspace.
#[derive(Debug, Eq, Hash, PartialEq, Serialize)]
pub struct LayoutUpdateInfo<'a> {
    /// A mutable list of all tiles.
    pub tiles: &'a mut Vec<Tile>,
    /// The bounding box of the workspace. Tiles should only be layed out inside these boundaries!
    pub workspace_bbox: BBox,
}

/// A layout.
#[typetag::serde(tag = "layout")]
pub trait Layout: DynClone + Debug + Send + Sync {
    /// Returns the metadata of the layout.
    fn metadata(&self) -> LayoutMeta;
    /// Invalidates the layout and makes re-calculations possible.
    fn invalidate(&mut self);
    /// Returns whether the layout is dirty or not. Dirty layouts need re-calculations.
    fn is_dirty(&self) -> bool;
    /// Lays out tiles by changing their bbox.
    fn layout<'a>(&mut self, update_info: &'a mut LayoutUpdateInfo<'a>);
}

dyn_clone::clone_trait_object!(Layout);
