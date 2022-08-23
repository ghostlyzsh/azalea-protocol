use buffer::buffer::ByteBuf;

pub struct StatusRequest {
    buffer: ByteBuf
}

crate::create_packets!(
    StatusRequest {
        id 0x00;
    }
);
