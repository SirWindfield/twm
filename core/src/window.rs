//! Windows are used to represent running programs of a user.

use crate::bbox::BBox;
use serde::{Deserialize, Serialize};

/// A window id.
pub type WindowId = u32;
/// A window handle.
pub type WindowHandle = i32;

/// A window.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Window {
    /// The unique id of a window.
    pub id: WindowId,
    /// The OS' window handle.
    pub handle: WindowHandle,
    /// The original bounding box of the window before resizing it using a
    /// layout.
    pub original_bbox: BBox,
}

impl Window {
    /// Creates an empty window.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a window.
    ///
    /// # Arguments
    ///
    /// - `id`: The window id.
    /// - `handle`: The window handle.
    /// - `original_bbox`: The original bounding box.
    pub fn with(id: WindowId, handle: WindowHandle, original_bbox: BBox) -> Self {
        Self {
            id,
            handle,
            original_bbox,
        }
    }
}
