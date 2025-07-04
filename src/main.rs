use anyhow::Result;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
mod user;
use user::{ChatMessage, User};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080\n");

    let (tx, _) = broadcast::channel::<ChatMessage>(100);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}\n", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            match User::new(socket, addr).await {
                Ok(mut user) => {
                    let join_message = ChatMessage {
                        sender: None,
                        content: format!("{} has joined the chat", user.name),
                    };
                    let _ = tx.send(join_message);

                    if let Err(e) = user.handle_client(&tx, &mut rx).await {
                        eprintln!("Error in client handler: {:?}\n", e);
                    }

                    let leave_message = ChatMessage {
                        sender: None,
                        content: format!("{} has left the chat", user.name),
                    };
                    let _ = tx.send(leave_message);
                }
                Err(e) => {
                    eprintln!("Error initializing user: {:?}\n", e);
                }
            }
        });
    }
}
