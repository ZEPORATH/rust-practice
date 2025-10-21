use clap::{Parser, Subcommand};
use std::path::PathBuf;

use basic_file_server::client::ClientCli;
use basic_file_server::Server;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server { addr: String, mount: PathBuf },
    Client { #[command(flatten)] opts: ClientCli },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Server { addr, mount } => {
            let mut server = Server::new(&addr, mount);
            server.run()
        }
        Commands::Client { opts } => {
            // Start async-std runtime for client
            async_std::task::block_on(basic_file_server::client::Client::run_cli(opts)).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}