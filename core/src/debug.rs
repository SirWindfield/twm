//! Contains debug-related logic.

use serde::Deserialize;
use std::path::PathBuf;

/// Returns the path of the workspace that the crate is in.
///
/// Note that this does not do any checks and assumes that this crate (`twm-core`) is inside a workspace.
pub(crate) fn workspace_path() -> PathBuf {
    #[derive(Deserialize)]
    struct Manifest {
        workspace_root: String,
    }

    let output = std::process::Command::new(env!("CARGO"))
        .arg("metadata")
        .arg("--format-version=1")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    let manifest: Manifest = serde_json::from_slice(&output.stdout).unwrap();
    PathBuf::from(manifest.workspace_root)
}
