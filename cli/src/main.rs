use crate::cli::{App, InfoSubCommand};
use clap::Clap;
use jsonrpc_core_client::transports::ipc::connect;
use tokio::runtime::Runtime;

mod cli;

const PIPE_NAME: &'static str = "\\\\.\\pipe\\twm-pipe";

fn main() {
    let opts: App = cli::App::parse();
    let mut runtime = Runtime::new().unwrap();
    let reactor = runtime.reactor().clone();
    let client = runtime
        .block_on(connect::<_, twm_protocol::gen_client::Client>(PIPE_NAME, &reactor).unwrap())
        .unwrap();

    match opts {
        App::Info(sub_command) => match sub_command {
            InfoSubCommand::Get { name } => match name.as_ref() {
                "twm.version" => {
                    let fut = client.protocol_version();
                    let res = runtime.block_on(fut).unwrap();
                    println!("{}", serde_json::to_string(&res).unwrap());
                }
                "twm.tiles.count" => {
                    let fut = client.tiles_count();
                    let res = runtime.block_on(fut).unwrap();
                    println!("{}", serde_json::to_string(&res).unwrap());
                }
                "twm.tiles.focused" => {
                    let fut = client.focused_tile();
                    let res = runtime.block_on(fut).unwrap();
                    println!("{}", serde_json::to_string(&res).unwrap());
                },
                "twm.workspaces.focused" => {
                    let fut = client.focused_workspace();
                    let res = runtime.block_on(fut).unwrap();
                    println!("{}", serde_json::to_string(&res).unwrap());
                },
                "twm.workspaces.count" => {
                    let fut = client.workspaces_count();
                    let res = runtime.block_on(fut).unwrap();
                    println!("{}", serde_json::to_string(&res).unwrap());
                },
                _ => unimplemented!(),
            },
        },
    }
}
