use async_std::io::stdin;
use futures::io;
use peer_piper::PeerPipe;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut link = PeerPipe::new()?;
    io::copy(stdin(), &mut link).await?;
    Ok(())
}
