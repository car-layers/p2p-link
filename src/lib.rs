use futures::io;
use libp2p::{identity, mdns::Mdns, swarm::Swarm};
use std::{
    error::Error,
    pin::Pin,
    task::{Context, Poll},
};

pub struct PeerPipe(Swarm<Mdns>);

impl PeerPipe {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let id_keys = identity::Keypair::generate_ed25519();
        Self::new_peer(id_keys)
    }

    pub fn new_peer(keys: identity::Keypair) -> Result<Self, Box<dyn Error>> {
        let peer_id = keys.public().into_peer_id();
        let transport = libp2p::build_tcp_ws_secio_mplex_yamux(keys)?;
        let behaviour = Mdns::new()?;
        let mut swarm = Swarm::new(transport, behaviour, peer_id);
        Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;
        // loop {
        //     let event = swarm.next_event().await;
        //     match event {
        //         SwarmEvent::NewListenAddr(addr) => println!("Listening on {}", addr),
        //         _ => println!("{:?}", event),
        //     }
        // }

        Ok(Self(swarm))
    }
}

impl io::AsyncWrite for PeerPipe {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
