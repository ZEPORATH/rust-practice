use std::env;
use std::process;

use basic_server_client::client;

use {basic_server_client::Client, basic_server_client::Server};



fn print_usage_and_exit() -> ! {
    eprintln!("Usage: cargo run -- [server|client] [addr]");
    eprintln!("Examples:");
    eprintln!(" cargo run -- server 127.0.0.1:4000");
    eprintln!(" cargo run -- client 127.0.0.1:4000");
    process::exit(1);
}

fn main() {
    let mut arg = env::args().skip(1);
    let mode = arg.next().unwrap_or_else(|| {print_usage_and_exit()});
    let addr = arg.next().unwrap_or_else(|| "127.0.0.0:4444".to_string());
    match mode.as_str() {
        "server" => {
            let server  = Server::New(&addr);
            server.WaitForClient();
        }
        "client" => {
            let client = Client::New(&addr);
            client.run();
        }
        _=>{
            eprint!("invalid mode entered: {}", mode.as_str());
            print_usage_and_exit();
        }
    }
}
