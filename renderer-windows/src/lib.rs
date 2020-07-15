use log::error;
use twm_core::config::Config;
use twm_core::platform::Renderer;
use twm_core::tile::Tile;
use twm_core::workspace::Workspace;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{SetWindowPos, SWP_NOSENDCHANGING};

pub struct WindowsRenderer;

impl WindowsRenderer {
    fn tile_diagnostics(&self, tile: &Tile) -> String {
        format!("Tile[id={},handle={}]", tile.id, tile.window.handle).to_string()
    }
}

impl Renderer for WindowsRenderer {
    fn init(&mut self, _config: &Config) {}

    fn render(&self, workspace: &Workspace) {
        for tile in workspace.tiles() {
            unsafe {
                let result = SetWindowPos(
                    tile.window.handle as HWND,
                    std::ptr::null_mut(),
                    tile.bbox.x,
                    tile.bbox.y,
                    tile.bbox.width,
                    tile.bbox.height,
                    SWP_NOSENDCHANGING,
                );

                if result == 0 {
                    error!(
                        "Failed to set bounding box for {}",
                        self.tile_diagnostics(tile)
                    );
                }
            }
        }
    }

    fn shutdown(&mut self) {}
}
