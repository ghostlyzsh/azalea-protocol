use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::packet::Packet;

pub struct PacketRegistry {
    register: Arc<RwLock<HashMap<i32, Box<dyn Packet>>>>,
}

impl PacketRegistry {
    pub async fn register_packet<T>(&mut self, id: i32, packet: Box<dyn Packet>) {
        let mut w = self.register.write().await;
        (*w).insert(id, packet);
    }

    pub async fn get_registry(&self) {
        self.register.read().await;
    }
}
