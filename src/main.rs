use async_std::io::stdin;
use peer_piper::PeerPipe;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init()?;
    PeerPipe::new()?.connect(stdin()).await
}
