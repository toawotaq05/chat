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
mod server;
mod client;

#[tokio::main]
async fn main() {
    if let Err(e) = server::run_server().await {
        eprintln!("Server error: {}", e);
    }
}
