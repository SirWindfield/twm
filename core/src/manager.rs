//! The `Manager` is responsible for managing all workspaces and keeping track of the active.

use crate::workspace::{Workspace, WorkspaceId};

/// A manager.
#[derive(Clone, Debug, Default)]
pub struct Manager {
    /// The id of the focused `Workspace`. `None` if no `Workspace` is focused.
    pub focused_workspace_id: Option<WorkspaceId>,
    /// The list of workspaces.
    pub workspaces: Vec<Workspace>,
}

impl Manager {
    /// Creates a default manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a manager.
    ///
    /// # Arguments
    ///
    /// - `workspaces`: The `Workspace`s managed by this manager.
    pub fn with(workspaces: Vec<Workspace>) -> Self {
        Self {
            workspaces,
            ..Self::default()
        }
    }

    /// Returns a reference to the focused workspace.
    ///
    /// # Returns
    ///
    /// `Some(&Workspace)` if a workspace is focused, `None` otherwise.
    pub fn focused_workspace(&self) -> Option<&Workspace> {
        if let Some(focused_workspace_id) = self.focused_workspace_id {
            if let Some(workspace) = self
                .workspaces
                .iter()
                .find(|ws| ws.id == focused_workspace_id)
            {
                return Some(workspace);
            }
        }

        None
    }

    /// Returns a mutable reference to the focused workspace.
    ///
    /// # Returns
    ///
    /// `Some(&mut Workspace)` if a workspace is focused, `None` otherwise.
    pub fn focused_workspace_mut(&mut self) -> Option<&mut Workspace> {
        if let Some(focused_workspace_id) = self.focused_workspace_id {
            if let Some(workspace) = self
                .workspaces
                .iter_mut()
                .find(|ws| ws.id == focused_workspace_id)
            {
                return Some(workspace);
            }
        }

        None
    }
}
