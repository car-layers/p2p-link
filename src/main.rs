use libp2p::{
    identity,
    ping::{Ping, PingConfig},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let transport = libp2p::build_development_transport(id_keys)?;
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    if let Some(addr) = std::env::args().nth(1) {
        let remote = addr.parse()?;
        Swarm::dial_addr(&mut swarm, remote)?;
        println!("Dialed {}", addr)
    }

    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;
    loop {
        let event = swarm.next_event().await;
        match event {
            SwarmEvent::NewListenAddr(addr) => println!("Listening on {}", addr),
            _ => println!("{:?}", event),
        }
    }
}
