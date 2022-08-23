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

    pub fn get_packet_type(&self, id: i32, buffer: ByteBuf) -> Result<PacketType, ()> {
        let value = match self.register.get(&id) {
            Some(n) => n,
            None => { println!("Invalid Packet ID {}", id); return Err(()); }
        };
        Ok(value(buffer))
    }
}

#[macro_export]
macro_rules! create_registry {
    ( $($packet_type:ident),* ) => {
        {
            let registry = $crate::packet_registry::PacketRegistry::new();
            $(
                $packet_type::register_packet(registry);
            )*
            registry
        }
    };
}
