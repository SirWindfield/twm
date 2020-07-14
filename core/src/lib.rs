//!

#![warn(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)]

pub mod bbox;
pub mod config;
mod debug;
pub mod display;
pub mod layout;
pub mod manager;
pub mod tile;
pub mod util;
pub mod window;
pub mod workspace;

pub use config::toml;
pub use layout::DynClone;

#[cfg(test)]
mod tests {
    use tracing::Level;

    pub fn init_tracing() {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("no global subscriber has been set");
    }

    #[test]
    fn test_l() {
        init_tracing();

        // ... init stuff
        tracing::info!(foo = 1, bar = 2);
    }
}
