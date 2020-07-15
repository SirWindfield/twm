use crate::keys::Listener;
use crate::platform::discover_main_display;
use keyboard_shortcut_parser::{KeyModifier, KeySpecial};
use twm_core::config::{config_file_path, parse};

use crate::twm::Twm;

use anyhow::Result;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tracing::info;
use tracing::Level;
use twm_core::bbox::{BBox, SplitDirection};
use twm_core::display::Display;
use winapi::_core::sync::atomic::AtomicBool;
use std::io::Read;

const PIPE_NAME: &'static str = "\\\\.\\pipe\\twm-pipe";

mod config;
mod keys;
mod platform;
mod twm;

// let mut twm = Twm::new();
// twm.new_ws();
// let mut io = IoHandler::new();
// io.extend_with(twm.to_delegate());
//
// let builder = ServerBuilder::new(io);
// let server = builder.start(PIPE_NAME).expect("Couldn't open pipe");
// server.wait();

fn init_tracing() -> Result<()> {
    use tracing_log::env_logger::BuilderExt;

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    let mut builder = env_logger::Builder::new();
    builder
        .filter(Some("twm-core"), log::LevelFilter::Trace)
        .emit_traces() // from `tracing_log::env_logger::BuilderExt`
        .try_init()?;
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[tracing::instrument]
fn init_twm(u: usize) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("called");
        r.store(false, Ordering::SeqCst);
    })
    .expect("error setting ctrl-c handler");

    let mut twm = Twm::new();

    // Configure twm if a config file could be found.
    if let Some(config_path) = config_file_path() {
        info!("Found config: {}", config_path.display());
        println!("lol");
        let mut file = std::fs::File::open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        println!("{}", &content);

        twm.config = Some(twm_core::toml::from_str(&content).unwrap());
    }

    let display: Display = discover_main_display();
    let width = display.bbox.width;
    let height = display.bbox.height;
    info!("Display: {}x{}", width, height);

    twm.new_ws();
    BBox::equal_split(display.bbox, 4, SplitDirection::Vertical);

    let mut listener = Listener::new();
    let res = listener.register_hook(
        (KeyModifier::CONTROL | KeyModifier::ALT).bits(),
        KeySpecial::ENTER.bits() as u32,
        hook,
    );

    if let Ok(id) = res {
        println!("registered with id {}", id);
        listener.listen(running);
    }

    println!("OK");
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;
    info!("Initialized tracing framework");

    init_twm(1);
    info!("Initialized twm@{}", version = env!("CARGO_PKG_VERSION"));

    Ok(())
}

fn hook() {
    println!("pressed");
}
