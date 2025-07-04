use anyhow::Result;
use tokio::net::TcpListener;
mod user;
use user::User;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let user = User::new(socket, addr).await;

            user.handle_client().await;
        });
    }
}
