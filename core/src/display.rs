//!

use crate::bbox::BBox;
use serde::{Deserialize, Serialize};

/// A display id.
pub type DisplayId = u32;

/// A display represents a monitor of an user. These can either be physical or virtual ones.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Display {
    /// The unique id of the display.
    pub id: DisplayId,
    /// The bounding box of the display.
    pub bbox: BBox,
}

impl Display {
    /// Creates an empty display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new display with specific values.
    ///
    /// # Arguments
    ///
    /// - `id`: The id of the display
    /// - `bbox`: The bounding box of the display.
    pub fn with(id: DisplayId, bbox: BBox) -> Self {
        Self { id, bbox }
    }
}
