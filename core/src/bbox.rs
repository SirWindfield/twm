//! Bounding boxes are used to hold the position and size of windows inside the tiling window manager.

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::trace;

/// The result of an horizontal split.
///
/// Splitting a bounding box horizontally results in two new bounding boxes: an upper one and a lower one.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct HorizontalSplit(BBox, BBox);

impl HorizontalSplit {
    /// Creates an empty `HorizontalSplit`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `HorizontalSplit` holding the given `upper` and `lower` bounding boxes.
    pub fn with(upper: BBox, lower: BBox) -> Self {
        Self(upper, lower)
    }

    /// Returns the upper bounding box.
    pub fn upper(&self) -> BBox {
        self.0
    }

    /// Returns the lower bounding box.
    pub fn lower(&self) -> BBox {
        self.1
    }
}

/// The result of an vertical split.
///
/// Splitting a bounding box vertically results in two new bounding boxes: a left one and a right one.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VerticalSplit(BBox, BBox);

impl VerticalSplit {
    /// Creates an empty vertical split.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `HorizontalSplit` holding the given `left` and `right` bounding boxes.
    pub fn with(left: BBox, right: BBox) -> Self {
        Self(left, right)
    }

    /// Returns the left bounding box.
    pub fn left(&self) -> BBox {
        self.0
    }

    /// Returns the right bounding box.
    pub fn right(&self) -> BBox {
        self.1
    }
}

/// A bounding box.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BBox {
    /// The x-position of the bounding box.
    pub x: i32,
    /// The y-position of the bounding box.
    pub y: i32,
    /// The width of the bounding box.
    pub width: i32,
    /// The height of the bounding box.
    pub height: i32,
}

impl BBox {
    /// Creates an empty bounding box.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new bounding box with specific values.
    ///
    /// # Arguments
    ///
    /// - `x`: The x-position of the bounding box.
    /// - `y`: The y-position of the bounding box.
    /// - `width`: The width of the bounding box.
    /// - `height`: The height of the bounding box.
    pub fn with(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Equally splits the surface area of a bounding box, returning the resulting bounding boxes.
    ///
    /// # Arguments
    ///
    /// - `root`: The bounding box that gets split up.
    /// - `number_of_bboxes`: The number of bounding boxes that the given root bounding box should be split into.
    /// - `split_direction`: Whether to split the bounding box horizontally or vertically.
    ///
    /// # Special cases
    ///
    /// - If `number_of_bboxes` is 0, this function is a no-op and returns an empty `Vec` with an initial capacity of zero.
    #[tracing::instrument]
    pub fn equal_split(
        root: BBox,
        number_of_bboxes: usize,
        split_direction: SplitDirection,
    ) -> Vec<BBox> {
        // TODO: check if number_of_bboxes(x) > width/height, as we can't move x times if the with is smaller than x
        trace!("equal_split");
        if number_of_bboxes == 0 {
            return Vec::with_capacity(0);
        }

        let mut bboxes = Vec::with_capacity(number_of_bboxes);

        for i in 0..number_of_bboxes {
            let bbox = match split_direction {
                SplitDirection::Horizontal => {
                    let height_per_part = root.height / number_of_bboxes as i32;
                    BBox::with(
                        root.x,
                        (i * height_per_part as usize) as i32,
                        root.width,
                        height_per_part,
                    )
                }
                SplitDirection::Vertical => {
                    let width_per_part = root.width / number_of_bboxes as i32;
                    BBox::with(
                        (i * width_per_part as usize) as i32,
                        root.y,
                        width_per_part,
                        root.height,
                    )
                }
            };
            bboxes.push(bbox);
        }

        bboxes
    }

    /// Splits the current bounding box horizontally and returns the result.
    #[tracing::instrument]
    pub fn horizontal_split(&self) -> HorizontalSplit {
        let upper = BBoxBuilder::from(*self).height(self.height / 2).build();

        // In case that the height is actually odd.
        let lower_height = self.height - upper.height;
        let lower_new_y = self.y + upper.height;

        let lower = BBoxBuilder::from(*self)
            .y(lower_new_y)
            .height(lower_height)
            .build();
        HorizontalSplit::with(upper, lower)
    }

    /// Splits the current bounding box vertically and returns the result.
    #[tracing::instrument]
    pub fn vertical_split(&self) -> VerticalSplit {
        let left = BBoxBuilder::from(*self).width(self.width / 2).build();

        // In case that the width is actually odd.
        let right_width = self.width - left.width;
        let right_new_x = self.x + left.width.abs();

        let right = BBoxBuilder::from(*self)
            .x(right_new_x)
            .width(right_width)
            .build();
        VerticalSplit::with(left, right)
    }
}

impl fmt::Display for BBox {
    /// Prints the bounding box as a human-readable string. The format is `{width}x{height}@(x,y)`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}@({},{})", self.width, self.height, self.x, self.y)
    }
}

#[cfg(test)]
impl Arbitrary for BBox {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        BBox {
            x: i32::arbitrary(g),
            y: i32::arbitrary(g),
            // For now, only positive heights and widths are allowed
            // TODO: is this always the case IRL?
            width: i32::arbitrary(g).abs(),
            height: i32::arbitrary(g).abs(),
        }
    }
}

/// A builder for creating bounding boxes.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct BBoxBuilder {
    bbox: BBox,
}

impl BBoxBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new builder and pre-populates the fields using the bounding box's values.
    pub fn from(bbox: BBox) -> Self {
        Self { bbox }
    }

    /// Sets the x-position.
    pub fn x(&mut self, x: i32) -> &mut Self {
        self.bbox.x = x;
        self
    }

    /// Sets the y-position.
    pub fn y(&mut self, y: i32) -> &mut Self {
        self.bbox.y = y;
        self
    }

    /// Sets the height value.
    pub fn height(&mut self, height: i32) -> &mut Self {
        self.bbox.height = height;
        self
    }

    /// Sets the width value.
    pub fn width(&mut self, width: i32) -> &mut Self {
        self.bbox.width = width;
        self
    }

    /// Creates a bounding box and consuming the builder.
    pub fn build(self) -> BBox {
        self.bbox
    }
}

/// The direction a split can occur.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SplitDirection {
    /// A horizontal split splits a bounding box into an `upper` and `lower` part.
    Horizontal,
    /// A vertical split splits a bounding box into a `left` and `right` part.
    Vertical,
}

impl Default for SplitDirection {
    /// Returns the default split direction (Horizontal).
    fn default() -> Self {
        SplitDirection::Horizontal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_horizontal_split() {
        let bbox = BBox::with(0, 0, 1920, 1080);
        let splitted = bbox.horizontal_split();

        let upper = BBox::with(0, 0, 1920, 540);
        let lower = BBox::with(0, 540, 1920, 540);
        let expected = HorizontalSplit::with(upper, lower);

        assert_eq!(expected, splitted);
    }

    #[test]
    fn test_vertical_split() {
        let bbox = BBox::with(0, 0, 1920, 1080);
        let splitted = bbox.vertical_split();

        let left = BBox::with(0, 0, 960, 1080);
        let right = BBox::with(960, 0, 960, 1080);
        let expected = VerticalSplit::with(left, right);

        assert_eq!(expected, splitted);
    }
}
