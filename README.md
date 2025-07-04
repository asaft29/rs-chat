# TCP Chat Server
A simple TCP chat server built in Rust using [tokio](https://tokio.rs/) for asynchronous handling and broadcast channels for message distribution.

I just wanted to start using async and see what Rust has to offer pretty much.

Messages are sent to all connected clients via broadcast, except for the sender.
## Run the server
```bash
cargo run --release
```
Server starts on `127.0.0.1:8080`
## Testing
Connect with telnet:
```bash
telnet 127.0.0.1 8080
```
Open multiple terminals to test multi-client chat.
