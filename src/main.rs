use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
mod user;
use user::{ChatMessage, User};

const SIZE: usize = 2048;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080\n");

    let set = Arc::new(tcp_server::ConcurrentSet::new());

    let (tx, _) = broadcast::channel::<ChatMessage>(SIZE);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}\n");

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let set = set.clone();

        tokio::spawn(async move {
            match User::new(socket, addr, set).await {
                Ok(mut user) => {
                    let join_message = ChatMessage {
                        sender: None,
                        content: format!("{} has joined the chat\n", user.name),
                    };
                    let _ = tx.send(join_message);

                    if let Err(e) = user.handle_client(&tx, &mut rx).await {
                        eprintln!("Error in client handler: {e:?}\n");
                    }

                    let leave_message = ChatMessage {
                        sender: None,
                        content: format!("{} has left the chat", user.name),
                    };
                    let _ = tx.send(leave_message);
                }
                Err(e) => {
                    eprintln!("Error initializing user: {e:?}\n");
                }
            }
        });
    }
}
