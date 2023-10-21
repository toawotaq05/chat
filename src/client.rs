//! Client Initialization:
//! 
//! Set up the client socket and establish a connection to the server.
//! User Interaction:
//! 
//! Handle user input for sending messages and display incoming messages from the server.
//! Message Transmission and Reception:
//! 
//! Send messages to the server and receive messages from the server, utilizing Serde for serialization and deserialization.
//! Asynchronous Task Handling:
//! 
//! Similar to the server, utilize Tokio for handling asynchronous tasks like reading from the socket and processing user input concurrently.
//! Error Handling:
//! 
//! Handle client-specific errors and ensure a smooth user experience.
//! Interaction Between Components:


use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn run_client() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to the server!");

    // Sending a message
    let message = b"Hello, server!";
    stream.write_all(message).await?;

    // Receiving a response
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[0..n]);
    println!("Received: {}", response);

    Ok(())
}