use async_std::io::{stdin, stdout};
use peer_piper::PeerPiper;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init()?;
    PeerPiper::new()
        .with_in(&stdin())
        .with_out(&stdout())
        .pipe()
        .await
}

