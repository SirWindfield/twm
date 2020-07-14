//!

use crate::layout::{Layout, LayoutUpdateInfo, LayoutMeta};
use serde::{Deserialize, Serialize};

///
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MiddleLayout {
    dirty: bool,
}

impl MiddleLayout {
    fn layout0<'a>(&mut self, _update_info: &'a mut LayoutUpdateInfo<'a>) {

    }
}

#[typetag::serde]
impl Layout for MiddleLayout {
    fn metadata(&self) -> LayoutMeta {
        LayoutMeta {
            name: "Middle Layout".to_string(),
        }
    }

    fn invalidate(&mut self) {
        self.dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn layout<'a>(&mut self, update_info: &'a mut LayoutUpdateInfo<'a>) {
        self.layout0(update_info)
    }
}