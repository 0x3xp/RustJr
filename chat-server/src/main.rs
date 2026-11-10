use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::time::timeout;

const MAX_CLIENTS: usize = 128;
const CLIENTS_TIMEOUT_SECS: u64 =30;

async fn handle_connection(stream: TcpStream, addr: SocketAddr,
    tx: broadcast::Sender<String>, mut rx: broadcast::Receiver<String>) -> Result<(), Box<dyn Error + Send + Sync>> {

    let(reader,mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"[::] Welcome to Rust broadcast Server! [::]\n").await?;

    let _ = tx.send(format!("[+] {} Joined to server\n", addr));

    loop {
        tokio::select! {
            read_result = timeout(Duration::from_secs(CLIENTS_TIMEOUT_SECS), reader.read_line(&mut line)) => {
                match read_result {
                    Err(_elapsed) => {
                        let _ = writer.write_all(b"[!] Disconnected due to inactivity - Slowloris Defend\n").await;
                        eprintln!("[-] Slowloris timeout reached for {}", addr);
                        break;
                    }
                    Ok(res) => {
                        let bytes_read = res?;
                        if bytes_read == 0 {
                            break;
                        }

                        let broadcast_msg = format!("[{}] {}", addr, line);
                        let _ = tx.send(broadcast_msg);
                        line.clear();
                    }
                }
            }

            Ok(msg) = rx.recv() => {
                if msg == "Server Shutdown" {
                    let _ = writer.write_all(b"[!] Server is shutting down. Good Bye!\n").await;
                    break;
                }
                if !msg.starts_with(&format!("[{}]", addr)) {
                    writer.write_all(msg.as_bytes()).await?;
                }
            }
        }
    }
            let _ = tx.send(format!("[-] {} left the server!\n",addr));
            Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bind_addr = "127.0.0.1:5050";
    let listener = TcpListener::bind(bind_addr).await?;
    println!("[*] Broadcast server running on {}", bind_addr);
    println!("[*] Press Ctrl+C to initiate graceful shutdown");

    let (tx, _) = broadcast::channel::<String>(MAX_CLIENTS);

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                let (stream, addr) = accept_result?;
                let tx = tx.clone();
                let rx = tx.subscribe();

                tokio::spawn(async move {
                     if let Err(e) = handle_connection(stream, addr, tx, rx).await {
                        eprintln!("[!] Connection error with {}: {}", addr, e);
                    }
                });
            }

            _ = tokio::signal::ctrl_c() => {
                println!("\n[!] Ctrl+C detected! Initiating graceful shutdown...");

                let _ = tx.send("SERVER_SHUTDOWN".to_string());

                tokio::time::sleep(Duration::from_millis(200)).await;
                println!("[*] All sockets notified. Server stopped cleanly. Bye!");
                break;
            }
        }
    }
    Ok(())
}
