//! Contains model-independent enums, structs and traits.

use serde::{Deserialize, Serialize};

/// A general direction.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Direction {
    #[allow(missing_docs)]
    Left,
    #[allow(missing_docs)]
    Right,
    #[allow(missing_docs)]
    Up,
    #[allow(missing_docs)]
    Down,
}

impl Default for Direction {
    /// Returns the default direction (Left).
    fn default() -> Self {
        Direction::Left
    }
}
