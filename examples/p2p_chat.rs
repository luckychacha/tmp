use std::borrow::Cow;

use anyhow::Result;
use libp2p::{
    core::upgrade,
    floodsub::{self, Floodsub, FloodsubEvent, Topic},
    futures::StreamExt,
    identity,
    mdns::{Mdns, MdnsEvent},
    noise,
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    yamux, NetworkBehaviour, PeerId, Swarm, Transport,
};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct ChatBehaviour {
    /// flood subscription，比较浪费带宽，gossipsub 是更好的选择
    floodsub: Floodsub,
    /// 本地节点发现机制
    mdns: Mdns,
}

impl ChatBehaviour {
    pub async fn new(id: PeerId) -> Result<Self> {
        Ok(Self {
            floodsub: Floodsub::new(id),
            mdns: Mdns::new(Default::default()).await?,
        })
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for ChatBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(msg) = event {
            let text = String::from_utf8_lossy(&msg.data);
            println!("{:?}: {:?}", msg.source, text);
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for ChatBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                // 把 mdns 发现的新的 peer 加入到 floodsub 的 view 中
                for (id, addr) in list {
                    println!("Got peer: {:?} with addr {:?}", &id, &addr);
                    self.floodsub.add_node_to_partial_view(id);
                }
            }
            MdnsEvent::Expired(list) => {
                // 把 mdns 发现的离开的 peer 加入到 floodsub 的 view 中
                for (id, addr) in list {
                    println!("Removed peer: {:?} with addr {:?}", &id, &addr);
                    self.floodsub.remove_node_from_partial_view(&id);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 如果带参数，当成一个 topic
    let name = match std::env::args().nth(1) {
        Some(arg) => Cow::Owned(arg),
        None => Cow::Borrowed("lobby"),
    };

    let topic = floodsub::Topic::new(name);

    let mut swarm = create_swarm(topic.clone()).await?;

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

    let mut stdin = BufReader::new(stdin()).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(topic.clone(), line.as_bytes());
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Listening on: {:?}", address);
                }

            }
        }
    }
}

async fn create_swarm(topic: Topic) -> Result<Swarm<ChatBehaviour>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    let noise_keys = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&id_keys)?;

    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let mut behaviour = ChatBehaviour::new(peer_id.clone()).await?;

    behaviour.floodsub.subscribe(topic.clone());

    let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    Ok(swarm)
}
