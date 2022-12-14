use buffer::buffer::ByteBuf;

pub struct StatusRequest {
    buffer: ByteBuf
}

pub struct PingRequest {
    buffer: ByteBuf
}

crate::create_packets!(
    StatusRequest {
        id 0x00;
    }
    PingRequest {
        id 0x01;
    }
);
