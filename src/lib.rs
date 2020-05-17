//! Peer piper connects peers in the local network and acts as bi-directional pipe
//! that sends one peer's input to another's output and the other way around.
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

/// PeerPiper implements AsyncRead and AsyncWrite to be a full-duplex
/// channel among connected peers in the local network.
pub struct PeerPiper<'a> {
    swarm: Swarm<Mdns>,
    input: Option<&'a dyn io::AsyncRead>,
    outputs: Vec<&'a dyn io::AsyncWrite>,
}

impl<'a> PeerPiper<'a> {
    /// Create a pipe with auto-generated peer identity
    pub fn new() -> Self {
        let id_keys = identity::Keypair::generate_ed25519();
        debug!("Generated peer keys {}", id_keys.public().into_peer_id());
        Self::new_peer(id_keys)
    }

    /// Create a new pipe with the supplied identity
    pub fn new_peer(keys: identity::Keypair) -> Self {
        let peer_id = keys.public().into_peer_id();
        let transport = libp2p::build_tcp_ws_secio_mplex_yamux(keys).unwrap();
        let behaviour = Mdns::new().unwrap();
        let swarm = Swarm::new(transport, behaviour, peer_id);
        PeerPiper {
            swarm,
            input: None,
            outputs: vec![],
        }
    }

    /// Data to transmit to connected peers
    pub fn with_in(mut self, reader: &'a impl io::AsyncRead) -> Self {
        self.input = Some(reader);
        self
    }

    /// Write incomming data to all the configured writers
    pub fn with_out(mut self, writer: &'a impl io::AsyncWrite) -> Self {
        self.outputs.push(writer);
        self
    }

    /// Start listening for incomming connections and handle data flow.
    pub async fn pipe(&mut self) -> Result<(), Box<dyn Error>> {
        Swarm::listen_on(&mut self.swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;
        loop {
            let event = self.swarm.next_event().await;
            match event {
                SwarmEvent::NewListenAddr(addr) => info!("Listening on {}", addr),
                _ => debug!("{:?}", event),
            }
        }
    }
}

impl<'a> io::AsyncRead for PeerPiper<'a> {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context,
        _buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        todo!()
    }
}

impl<'a> io::AsyncWrite for PeerPiper<'a> {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context, _buf: &[u8]) -> Poll<io::Result<usize>> {
        todo!()
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        todo!()
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
        todo!()
    }
}
