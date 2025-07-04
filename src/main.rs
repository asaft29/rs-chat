use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener}
};

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client disconnected");
                        break; 
                    }
                    Ok(n) => {
                        let received = String::from_utf8_lossy(&buf[..n]);
                        print!("Received: {}", received);

                        if let Err(e) = socket.write_all(received.as_bytes()).await {
                            eprintln!("Failed to write to socket; error = {:?}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket; error = {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}

