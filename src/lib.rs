use futures::io;
use libp2p::{
    identity,
    mdns::Mdns,
    swarm::{Swarm, SwarmEvent},
};
use log::{debug, info};
use std::{
    error::Error,
    pin::Pin,
    task::{Context, Poll},
};

pub struct PeerPipe(Swarm<Mdns>);

impl PeerPipe {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let id_keys = identity::Keypair::generate_ed25519();
        debug!("Generated peer keys {}", id_keys.public().into_peer_id());
        Self::new_peer(id_keys)
    }

    pub fn new_peer(keys: identity::Keypair) -> Result<Self, Box<dyn Error>> {
        let peer_id = keys.public().into_peer_id();
        let transport = libp2p::build_tcp_ws_secio_mplex_yamux(keys)?;
        let behaviour = Mdns::new()?;
        let swarm = Swarm::new(transport, behaviour, peer_id);
        Ok(Self(swarm))
    }

    pub async fn connect(&mut self, _reader: impl io::AsyncRead) -> Result<(), Box<dyn Error>> {
        Swarm::listen_on(&mut self.0, "/ip4/0.0.0.0/tcp/0".parse()?)?;
        loop {
            let event = self.0.next_event().await;
            match event {
                SwarmEvent::NewListenAddr(addr) => info!("Listening on {}", addr),
                _ => debug!("{:?}", event),
            }
        }
    }
}

impl io::AsyncWrite for PeerPipe {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context, _buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
