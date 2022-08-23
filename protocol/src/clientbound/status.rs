use buffer::buffer::ByteBuf;

pub struct StatusResponse {
    buffer: ByteBuf
}

pub struct PingResponse {
    buffer: ByteBuf
}

crate::create_packets!(
    StatusResponse {
        id 0x00;
    }
    PingResponse {
        id 0x01;
    }
);
