use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: usize,
    pub to: usize,
    pub data: Vec<u8>,
}

pub struct Network {
    nodes: usize,
    channels: Vec<mpsc::Sender<Message>>,
}

impl Network {
    pub fn new(nodes: usize) -> Self {
        let mut channels = Vec::new();
        for _ in 0..nodes {
            let (tx, _rx) = mpsc::channel(100);
            channels.push(tx);
        }
        Network { nodes, channels }
    }

    pub async fn send_message(&self, msg: Message) {
        if let Some(tx) = self.channels.get(msg.to) {
            tx.send(msg).await.unwrap();
        }
    }
}
