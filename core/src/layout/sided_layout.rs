//! A sided layout uses have of the workspace's bounding box for a `focused` tile. The rest of the space is evenly shared between the rest of the tiles.

use crate::bbox::{BBox, SplitDirection};
use crate::layout::{Layout, LayoutMeta, LayoutUpdateInfo};
use crate::tile::Tile;
use crate::util::Direction;
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

/// A `SidedBBox` splits a bounding box into two parts, the `sided` part, which is taken by the `focused` tile and the `rest` part, that is shared between the rest of the tiles.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct SidedBBox(BBox, BBox);

impl SidedBBox {
    /// Returns the `sided` bounding box.
    pub fn sided(&self) -> BBox {
        self.0
    }

    /// Returns the `rest` bounding box.
    pub fn rest(&self) -> BBox {
        self.1
    }
}

/// A sided layout implementation.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SidedLayout {
    /// Whether the layout is dirty or not.
    dirty: bool,
    /// The side that the focused tile is rendered to.
    pub side: Direction,
}

impl SidedLayout {
    /// Creates a default `SidedLayout`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `SidedLayout`.
    ///
    /// # Arguments
    ///
    /// - `side`: The side on which the focused tile is rendered to.
    pub fn with(side: Direction) -> Self {
        Self {
            side,
            ..Self::default()
        }
    }

    /// Returns the bounding boxes depending on which side the focused tile is rendered to.
    fn bbox_for_side(self, boundary: BBox) -> SidedBBox {
        match self.side {
            Direction::Left => {
                let splitted = boundary.vertical_split();
                SidedBBox(splitted.left(), splitted.right())
            }
            Direction::Right => {
                let splitted = boundary.vertical_split();
                SidedBBox(splitted.right(), splitted.left())
            }
            Direction::Up => {
                let splitted = boundary.horizontal_split();
                SidedBBox(splitted.upper(), splitted.lower())
            }
            Direction::Down => {
                let splitted = boundary.horizontal_split();
                SidedBBox(splitted.lower(), splitted.upper())
            }
        }
    }

    /// Returns the split direction used for splitting up the `rest` of the bounding box.
    ///
    /// # Returns
    ///
    /// For directions `Left` and `Right`, this function returns `SplitDirection::Horizontal`.
    /// For directions `Up` and `Down`, this function returns `SplitDirection::Vertical`.
    fn split_direction(self) -> SplitDirection {
        match self.side {
            Direction::Left | Direction::Right => SplitDirection::Horizontal,
            Direction::Up | Direction::Down => SplitDirection::Vertical,
        }
    }

    // The logic behind sided layout.
    #[tracing::instrument(skip(self, update_info))]
    fn layout0<'a>(&mut self, update_info: &'a mut LayoutUpdateInfo<'a>) {
        let display_ = update_info.workspace_bbox;
        let tiles: &mut Vec<Tile> = update_info.tiles;
        trace!("Aligning {} tiles inside of {:?}", tiles.len(), display_);

        // Create bounding boxes for the focused tile and the rest of the tiles.
        let sided_box = self.bbox_for_side(display_);
        let (side_bbox, rest_bbox) = (sided_box.sided(), sided_box.rest());
        trace!(side_bbox = %side_bbox, rest_bbox = %rest_bbox, "Calculated bounding boxes");

        // Calculate the number of remaining tiles.
        let number_of_non_sided_bboxes = tiles.len() - 1;
        trace!(
            "Splitting remaining space between {} tiles",
            number_of_non_sided_bboxes
        );
        let child_bboxes = BBox::equal_split(
            rest_bbox,
            number_of_non_sided_bboxes,
            self.split_direction(),
        );

        // Find the newest tile.
        let max_id_tile_id = tiles.iter().max_by_key(|t| t.id).unwrap().id;

        debug!("Applying new bounding boxes");
        for (index, tile) in tiles.iter_mut().enumerate() {
            if tile.id == max_id_tile_id {
                tile.bbox = side_bbox;
                trace!("Applied bounding box for side tile@{}", tile.id);
            } else {
                tile.bbox = child_bboxes[index];
                trace!("Applied bounding box for normal tile@{}", tile.id);
            }
        }
        trace!("Applied new bounding boxes");
    }
}

#[typetag::serde]
impl Layout for SidedLayout {
    fn metadata(&self) -> LayoutMeta {
        LayoutMeta {
            name: "Sided Layout".into(),
        }
    }

    fn invalidate(&mut self) {
        self.dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[tracing::instrument(skip(self, update_info))]
    fn layout<'a>(&mut self, update_info: &'a mut LayoutUpdateInfo<'a>) {
        if self.is_dirty() {
            trace!("Layout is dirty");

            if update_info.tiles.is_empty() {
                trace!("No tiles inside of workspace. Early return");
                return;
            }
            self.layout0(update_info);
            self.dirty = false;

            trace!("Layed out tiles. Marking as clean")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bbox::BBox;
    use crate::display::Display;
    use crate::tile::{Tile, TileId};
    use crate::window::Window;
    use crate::workspace::Workspace;

    pub fn init_tracing() {
        use tracing::Level;

        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("no global subscriber has been set");
    }

    fn generate_tiles(number_of_tiles: usize) -> Vec<Tile> {
        let mut vec = Vec::with_capacity(number_of_tiles);
        for i in 0..number_of_tiles {
            let tile = Tile::with(i as TileId, BBox::new(), Window::new());
            vec.push(tile);
        }

        vec
    }

    #[test]
    fn test_layout() {
        //init_tracing();

        let mut display = Display::new();
        display.bbox = BBox::with(0, 0, 1920, 1080);
        let mut workspace = Workspace::with(0, display);
        for tile in generate_tiles(3) {
            workspace.add_tile(tile);
        }

        workspace.layout.invalidate();
        workspace.layout();

        assert_eq!(true, true);
    }
}
