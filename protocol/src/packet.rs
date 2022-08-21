use crate::packet_registry::PacketRegistry;

pub trait Packet {
    fn register_packet(&self, registry: PacketRegistry);
    fn create_packet() -> Self where Self: Sized;
}

#[macro_export]
macro_rules! create_packets {
    ( $($packet_type:ty {
        id $packet_id:literal;
        $(protocol $protocol_version:literal;)?
    })* ) => {
        impl $crate::packet::Packet for $packet_type {
            fn register_packet(&self, registry: $crate::packet_registry::PacketRegistry) {
                registry.register_packet($packet_id, Box::new($packet_type { buffer::buffer::ByteBuf::new_write() }));
            }

            fn create_packet(buffer: buffer::buffer::ByteBuf)-> $packet_type
            where Self: Sized {
                $packet_type {buffer}
            }
        }
    };
}
