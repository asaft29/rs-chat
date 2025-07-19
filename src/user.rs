use anyhow::{Result, anyhow};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct User {
    pub name: String,
    socket: TcpStream,
    addr: SocketAddr,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub sender: Option<String>,
    pub content: String,
}

impl User {
    pub async fn new(mut socket: TcpStream, addr: SocketAddr) -> Result<Self> {
        let name = match Self::get_client_name(&mut socket).await {
            Ok(n) => n,
            Err(_) => {
                eprintln!(
                    "User with address {addr:?} exited before setting a name . Defaulting to 'Unknown'.\n"
                );
                String::from("Unknown")
            }
        };

        Ok(User { name, socket, addr })
    }

    async fn get_client_name(socket: &mut TcpStream) -> Result<String> {
        socket
            .write_all(
                b"\x1b[2J\x1b[1;1H\x1b[1;34mWelcome to RustChat!\x1b[0m\n\
          \x1b[1;33mEnter your guest name:\x1b[0m ",
            )
            .await?;

        let mut name_buffer = Vec::new();
        let mut read_buf = [0; 1];

        loop {
            match socket.read_exact(&mut read_buf).await {
                Ok(_) => {
                    let byte = read_buf[0];
                    if byte == b'\n' || byte == b'\r' {
                        break;
                    }
                    name_buffer.push(byte);
                }
                Err(e) => return Err(anyhow!("Error reading name from socket: {:?}", e)),
            }
        }

        let name = String::from_utf8_lossy(&name_buffer).trim().to_string();
        if name.is_empty() {
            Ok(String::from("Unknown"))
        } else {
            Ok(name)
        }
    }

    pub async fn handle_client(
        &mut self,
        tx: &broadcast::Sender<ChatMessage>,
        rx: &mut broadcast::Receiver<ChatMessage>,
    ) -> Result<()> {
        println!("New client [{} - {:?}] connected\n", self.name, self.addr);

        let mut buf = [0; 2048];

        loop {
            tokio::select! {
                result = self.socket.read(&mut buf) => {
                    let n = result?;
                    if n == 0 {
                        println!("Client [{} - {:?}] disconnected\n", self.name, self.addr);
                        break;
                    }

                    let received = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                    if !received.is_empty() {
                        let message = ChatMessage {
                            sender: Some(self.name.clone()),
                            content: received,
                        };
                        if let Some(ref valid) = message.sender {
                            println!("[{}]: {}", valid, message.content);
                        }
                        let _ = tx.send(message);
                    }
                }

                result = rx.recv() => {
                    match result {
                        Ok(msg) => {
                            match &msg.sender {
                                Some(valid_name) => {
                                    if valid_name == &self.name {

                                        // messages sent by the client are in green
                                        let to_send = format!("\x1b[2K\r\x1b[1;32mSent by me - {}\x1b[0m\n", msg.content);
                                        if let Err(e) = self.socket.write_all(to_send.as_bytes()).await {
                                            eprintln!("Failed to send to [{} - {:?}]; error = {:?}\n", self.name, self.addr, e);
                                            break;
                                        }
                                    } else {
                                        // for other clients is cyan
                                        let other_msg = format!("\x1b[36m[{}]: {}\x1b[0m", valid_name, msg.content);
                                        let padded = format!("{:>80}\n", other_msg);
                                        if let Err(e) = self.socket.write_all(padded.as_bytes()).await {
                                            eprintln!("Failed to send to [{} - {:?}]; error = {:?}\n", self.name, self.addr, e);
                                            break;
                                        }
                                    }
                                }
                                None => {
                                    //magenta
                                    let to_send = format!("\x1b[1;35m{}\x1b[0m\n", msg.content);
                                    if let Err(e) = self.socket.write_all(to_send.as_bytes()).await {
                                        eprintln!("Failed to send system message to [{} - {:?}]; error = {:?}\n", self.name, self.addr, e);
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Broadcast receive error for [{}]: {:?}\n", self.name, e);
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
