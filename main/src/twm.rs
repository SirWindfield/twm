use crate::config::Config;
use crate::platform::taskbar::Taskbar;
use std::sync::atomic::AtomicUsize;
use std::sync::RwLock;
use twm_core::manager::Manager;
use twm_core::tile::Tile;
use twm_core::workspace::Workspace;
use twm_protocol::{Error, ErrorCode, Result, Rpc};
use twm_core::layout::LayoutMeta;
use twm_core::DynClone;

#[derive(Debug)]
pub struct Twm {
    pub config: Option<Config>,
    pub last_workspace_id: AtomicUsize,
    pub manager: RwLock<Manager>,
    pub taskbar: Taskbar,
}

impl Twm {
    pub fn new() -> Self {
        let config = Config::default();
        println!("{}", twm_core::toml::to_string(&config).unwrap());

        Self {
            config: None,
            last_workspace_id: AtomicUsize::new(0),
            manager: RwLock::new(Manager::new()),
            taskbar: Taskbar::new(),
        }
    }

    pub fn new_ws(&mut self) {
        println!("after new_ws");
        let workspace = Workspace::new();
        println!("after ws");
        let mut m = self.manager.write().unwrap();
        println!("after lock");
        m.focused_workspace_id = Some(workspace.id);
        println!("after id");
        m.workspaces.push(workspace);
        println!("after push");
    }

    pub fn init(&self) {
        if let Some(config) = &self.config {
            if !config.taskbar.show {
                self.taskbar.hide();
            }
        }
    }

    pub fn uninit(&self) {
        if let Some(config) = &self.config {
            if !config.taskbar.show {
                self.taskbar.show();
            }
        }
    }
}

impl Drop for Twm {
    fn drop(&mut self) {
        self.uninit();
    }
}

impl Rpc for Twm {
    fn protocol_version(&self) -> Result<String> {
        Ok("1.0".into())
    }

    fn tiles_count(&self) -> Result<usize> {
        let read_ws = self.manager.read().unwrap();
        if let Some(workspace) = read_ws.focused_workspace() {
            return Ok(workspace.tiles().len());
        }

        Err(Error::new(ErrorCode::InternalError))
    }

    fn tile(&self, id: u32) -> Result<Tile> {
        let read_ws = self.manager.read().unwrap();
        if let Some(workspace) = read_ws.focused_workspace() {
            if let Some(tile) = workspace.tile_by_id(id) {
                return Ok(*tile);
            }
        }

        Err(Error::new(ErrorCode::InternalError))
    }

    fn layout(&self) -> Result<LayoutMeta> {
        if let Ok(manager) = self.manager.read() {
            if let Some(workspace) = manager.focused_workspace() {
                return Ok(workspace.layout.metadata())
            }
        }

        Err(Error::new(ErrorCode::InternalError))
    }

    fn focused_tile(&self) -> Result<Tile> {
        let read_ws = self.manager.read().unwrap();
        if let Some(workspace) = read_ws.focused_workspace() {
            if let Some(focused_tile) = workspace.focused_tile() {
                return Ok(*focused_tile);
            }
        }

        Err(Error::new(ErrorCode::InternalError))
    }

    fn focused_workspace(&self) -> Result<Workspace> {
        if let Ok(manager) = self.manager.read() {
            if let Some(workspace) = manager.focused_workspace() {
                return Ok(workspace.clone())
            }
        }

        Err(Error::new(ErrorCode::InternalError))
    }

    fn workspaces_count(&self) -> Result<usize> {
        if let Ok(manager) = self.manager.read() {
            return Ok(manager.workspaces.len());
        }

        Err(Error::new(ErrorCode::InternalError))
    }
}
