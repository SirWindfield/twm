pub use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;

use twm_core::tile::{Tile, TileId};
use twm_core::layout::LayoutMeta;
use twm_core::workspace::Workspace;

#[rpc]
pub trait Rpc {
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "tilesCount")]
    fn tiles_count(&self) -> Result<usize>;

    #[rpc(name = "tileById")]
    fn tile(&self, id: TileId) -> Result<Tile>;

    #[rpc(name = "activeLayout")]
    fn layout(&self) -> Result<LayoutMeta>;

    #[rpc(name = "focusedTile")]
    fn focused_tile(&self) -> Result<Tile>;

    #[rpc(name = "focusedWorkspace")]
    fn focused_workspace(&self) -> Result<Workspace>;

    #[rpc(name = "workspacesCount")]
    fn workspaces_count(&self) -> Result<usize>;
}
