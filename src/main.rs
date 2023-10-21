//! Command-Line Argument Parsing:
//! 
//! Parse command-line arguments to determine whether to initiate the server or client mode.
//! Potentially accept other configurations like server address, port number, or logging level.
//! Mode Dispatch:
//! 
//! Based on the parsed arguments, delegate to either server.rs or client.rs to run the server or client respectively.
//! Error Handling and Logging:
//! 
//! Handle any global errors and potentially set up logging configurations for the entire application.
mod client;
mod server;
use std::env;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) if arg == "server" => {
            // Run the server
            if let Err(e) = server::run_server().await {
                eprintln!("Server error: {}", e);
            }
        }
        Some(arg) if arg == "client" => {
            // Run the client
            if let Err(e) = client::run_client().await {
                eprintln!("Client error: {}", e);
            }
        }
        _ => {
            eprintln!("Usage: {} [server|client]", args[0]);
        }
    }
}