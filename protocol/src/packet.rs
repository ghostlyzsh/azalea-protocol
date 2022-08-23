use crate::packet_registry::PacketRegistry;
use crate::serverbound::status;

pub trait Packet {
    fn register_packet(&self, registry: &'static mut PacketRegistry);
    fn new(buffer: buffer::buffer::ByteBuf) -> PacketType;
}

#[macro_export]
macro_rules! create_packets {
    ( $($packet_type:ident {
        id $packet_id:literal;
        $(protocol $protocol_version:literal;)?
    })* ) => {
        $(
        impl $crate::packet::Packet for $packet_type {
            fn register_packet(&self, registry: &'static mut $crate::packet_registry::PacketRegistry) {
                registry.register_packet($packet_id, Self::new);
            }

            fn new(buffer: buffer::buffer::ByteBuf) -> $crate::packet::PacketType {
                $crate::packet::PacketType::$packet_type(Self {buffer})
            }
        }
        )*
    };
}

pub enum PacketType {
    StatusRequest(status::StatusRequest)
}
