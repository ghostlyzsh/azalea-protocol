use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::packet::{ PacketType };
use buffer::buffer::ByteBuf;

pub struct PacketRegistry {
    register: HashMap<i32, fn(ByteBuf) -> PacketType>,
}

impl PacketRegistry {
    pub fn register_packet(&mut self, id: i32, handle: fn(ByteBuf) -> PacketType) {
        self.register.insert(id, handle);
    }
    
    pub fn new() -> Self {
        Self {
            register: HashMap::new()
        }
    }
}
