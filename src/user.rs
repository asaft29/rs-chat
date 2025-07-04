use anyhow::{Result, anyhow};
use std::{net::SocketAddr, ops::Deref};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct User {
    name: String,
    socket: TcpStream,
    addr: SocketAddr,
}

impl User {
    pub async fn new(mut socket: TcpStream, addr: SocketAddr) -> Self {
        let name = match Self::get_client_name(&mut socket).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error getting name from {addr:?}: {e:?}. Defaulting to 'Unknown'.");
                String::from("Unknown")
            }
        };

        User { name, socket, addr }
    }

    async fn get_client_name(socket: &mut TcpStream) -> Result<String> {
        socket.write_all(b"Enter your guest name: ").await?;

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

    pub async fn handle_client(mut self) {
        println!("[{} - {:?}] connected", self.name, self.addr);

        let mut buf = [0; 2048];
        loop {
            match self.socket.read(&mut buf).await {
                Ok(0) => {
                    println!("[{} - {:?}] disconnected", self.name, self.addr);
                    break;
                }
                Ok(n) => {
                    let received = String::from_utf8_lossy(&buf[..n]);
                    if !received.trim().is_empty() { 
                    println!("[{} - {:?}] : {}", self.name, self.addr, received.trim());
                    }

                    let response = format!("[{}]: ", self.name);
                    if let Err(e) = self.socket.write_all(response.as_bytes()).await {
                        eprintln!(
                            "Failed to write to socket for [{} - {:?}]; error = {:?}",
                            self.name, self.addr, e
                        );
                        break;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to read from socket for [{} - {:?}]; error = {:?}",
                        self.name, self.addr, e
                    );
                    break;
                }
            }
        }
    }
}
