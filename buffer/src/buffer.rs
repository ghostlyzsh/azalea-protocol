pub struct ByteBuf {
    buffer: Vec<u8>,
    index: usize,
}

impl ByteBuf {
    pub fn new_write() -> ByteBuf {
        ByteBuf {
            buffer: Vec::new(),
            index: 0,
        }
    }

    pub fn new_read(new_buffer: Vec<u8>) -> ByteBuf {
        ByteBuf {
            buffer: new_buffer,
            index: 0,
        }
    }

    pub fn read_var_int(&mut self) -> i32 {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = self.read_byte();
            value |= ((current_byte & 0x7F) as i32) << position;

            if (current_byte & 0x80) == 0 {break;}

            position += 7;

            // TODO: proper error handling for too large VarInt
            if position >= 32 {return -1;}
        }

        return value;
    }

    pub fn read_byte(&mut self) -> u8 {
        self.index+=1;
        self.buffer[self.index]
    }

    pub fn write_var_int(&mut self, arg_value: i32) {
        let mut value: u32 = {
            let bytes = arg_value.to_be_bytes();
            u32::from_be_bytes(bytes)
        };
        loop {
            if (value & !0x7F) == 0 {
                self.write_byte(value as u8);
                return;
            }

            self.write_byte(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }

    pub fn write_byte(&mut self, arg_value: u8) {
        self.buffer.push(arg_value);
    }
}
