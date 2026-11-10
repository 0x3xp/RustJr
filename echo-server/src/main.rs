use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};
use std::error::Error;

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer_addr = socket.peer_addr()?;
    println!("New connection from {}", peer_addr);

    let mut buffer = [0u8; 1024];

    loop {
        let read_bytes = socket.read(&mut buffer).await?;

        if read_bytes == 0 {
            println!("CLient disconnected: {}", peer_addr);
            return Ok(());
        }

        println!("Recieved {} bytes from {}", read_bytes, peer_addr);

        socket.write_all(&buffer[..read_bytes]).await?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bind_addr = "127.0.0.1:4444";
    let listener = TcpListener::bind(bind_addr).await?;
    println!("Echo server listening on: {}", bind_addr);

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("CLient error: {}", e);
            }
        });
    }
}
