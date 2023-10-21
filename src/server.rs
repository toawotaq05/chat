//! Server Initialization:
//! 
//! Set up the server socket, bind to the specified IP address and port, and start listening for incoming connections.
//! Connection Management:
//! 
//! Accept new client connections, manage active connections, and handle disconnections gracefully.
//! Message Handling:
//! 
//! Receive messages from clients, process or route them accordingly, and send responses or broadcast messages to clients.
//! Asynchronous Task Handling:
//! 
//! Utilize Tokio to manage asynchronous tasks like handling multiple clients concurrently.
//! Error Handling:
//! 
//! Deal with any server-specific errors, ensuring robust and resilient server operation.
//! 


use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn run_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server is listening on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted connection from {:?}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
