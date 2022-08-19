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

    pub fn read_var_i32(&mut self) -> i32 {
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

    pub fn read_var_i64(&mut self) -> i64 {
        let mut value: i64 = 0;
        let mut position: i32 = 0;
        let mut current_byte: u8;

        loop {
            current_byte = self.read_byte();
            value |= ((current_byte & 0x7F) as i64) << position;

            if (current_byte & 0x80) == 0 {break;}

            position += 7;

            // TODO: proper error handling for VarLong too big
            if position >= 64 {return -1;}
        }

        return value;
    }

    pub fn read_i64(&mut self) -> i64 {
        let mut value: u64 = 0;
        value |= ((self.read_byte() & 0xFF) as u64) << 56;
        value |= ((self.read_byte() & 0xFF) as u64) << 48;
        value |= ((self.read_byte() & 0xFF) as u64) << 40;
        value |= ((self.read_byte() & 0xFF) as u64) << 32;
        value |= ((self.read_byte() & 0xFF) as u64) << 24;
        value |= ((self.read_byte() & 0xFF) as u64) << 16;
        value |= ((self.read_byte() & 0xFF) as u64) << 8;
        value |= (self.read_byte() & 0xFF) as u64;
        return value as i64;
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut value: u32 = 0;
        value |= ((self.read_byte() & 0xFF) as u32) << 24;
        value |= ((self.read_byte() & 0xFF) as u32) << 16;
        value |= ((self.read_byte() & 0xFF) as u32) << 8;
        value |= (self.read_byte() & 0xFF) as u32;
        value as i32
    }

    pub fn read_i16(&mut self) -> i16 {
        let mut value: u16 = 0;
        value |= ((self.read_byte() & 0xFF) as u16) << 8;
        value |= (self.read_byte() & 0xFF) as u16;
        value as i16
    }

    pub fn read_u16(&mut self) -> u16 {
        self.read_i16() as u16
    }

    pub fn read_f32(&mut self) -> f32 {
        let mut value: u32 = 0;

        value |= ((self.read_byte() & 0xFF) as u32) << 24;
        value |= ((self.read_byte() & 0xFF) as u32) << 16;
        value |= ((self.read_byte() & 0xFF) as u32) << 8;
        value |= (self.read_byte() & 0xFF) as u32;
        f32::from_bits(value)
    }

    pub fn read_f64(&mut self) -> f64 {
        let mut value: u64 = 0;

        value |= ((self.read_byte() & 0xFF) as u64) << 56;
        value |= ((self.read_byte() & 0xFF) as u64) << 48;
        value |= ((self.read_byte() & 0xFF) as u64) << 40;
        value |= ((self.read_byte() & 0xFF) as u64) << 32;
        value |= ((self.read_byte() & 0xFF) as u64) << 24;
        value |= ((self.read_byte() & 0xFF) as u64) << 16;
        value |= ((self.read_byte() & 0xFF) as u64) << 8;
        value |= (self.read_byte() & 0xFF) as u64;
        f64::from_bits(value)
    }

    pub fn read_bool(&mut self) -> bool {
        let value: u8 = self.read_byte();

        if value == 0x01 {
            return true;
        } else {
            return false;
        }
    }

    pub fn read_string(&mut self) -> String {
        let length: i32 = self.read_var_i32();
        let bytes = self.read_bytes(length.try_into().unwrap());
        String::from_utf8(bytes).unwrap()
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut value = Vec::with_capacity(length);
        for _i in 0..length {
            value.push(self.read_byte());
        }
        value
    }

    pub fn read_byte(&mut self) -> u8 {
        self.index+=1;
        self.buffer[self.index]
    }

    pub fn write_var_i32(&mut self, arg_value: i32) {
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

    pub fn write_var_i64(&mut self, arg_value: i64) {
        let mut value: u64 = {
            let bytes = arg_value.to_be_bytes();
            u64::from_be_bytes(bytes)
        };
        loop {
            if value & !(0x7F as u64) == 0 {
                self.write_byte(value as u8);
            }

            self.write_byte(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }

    pub fn write_i64(&mut self, arg_value: i64) {
        let value: u64 = {
            let bytes = arg_value.to_be_bytes();
            u64::from_be_bytes(bytes)
        };
        for i in 0..8 {
            self.write_byte(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
    }

    pub fn write_i32(&mut self, arg_value: i32) {
        let value: u32 = {
            let bytes = arg_value.to_be_bytes();
            u32::from_be_bytes(bytes)
        };
        for i in 0..4 {
            self.write_byte(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
    }

    pub fn write_i16(&mut self, arg_value: i16) {
        let value: u16 = {
            let bytes = arg_value.to_be_bytes();
            u16::from_be_bytes(bytes)
        };
        for i in 0..2 {
            self.write_byte(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
    }

    pub fn write_u16(&mut self, arg_value: u16) {
        self.write_i16(arg_value as i16);
    }

    pub fn write_f32(&mut self, arg_value: f32) {
        unsafe {
            let value: u32 = std::mem::transmute::<f32, u32>(arg_value);
            for i in 0..4 {
                self.write_byte(((value >> (i * 8)) & 0xFF).try_into().unwrap());
            }
        }
    }

    pub fn write_f64(&mut self, arg_value: f64) {
        unsafe {
            let value: u64 = std::mem::transmute(arg_value);
            for i in 0..8 {
                self.write_byte(((value >> (i * 8)) & 0xFF).try_into().unwrap());
            }
        }
    }

    pub fn write_string(&mut self, mut arg_value: String) {
        self.write_var_i32(arg_value.len() as i32);
        unsafe {
            self.write_bytes(arg_value.as_mut_vec().to_vec());
        }
    }

    pub fn write_bytes(&mut self, arg_value: Vec<u8>) {
        for value in arg_value {
            self.write_byte(value);
        }
    }

    pub fn write_bool(&mut self, arg_value: bool) {
        if arg_value {
            self.write_byte(0x01);
        } else {
            self.write_byte(0x00);
        }
    }

    pub fn write_byte(&mut self, arg_value: u8) {
        self.buffer.push(arg_value);
    }
}
