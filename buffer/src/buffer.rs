use std::cmp::Ordering;

pub struct ByteBuf {
    buffer: Vec<BufTypes>,
}

impl ByteBuf {
    pub fn read(&self, index: usize) -> BufTypes{
        self.buffer[index].clone()
    }

    pub fn write(&mut self, index: usize, value: BufTypes) {
        self.buffer.insert(index, value);
    }
}

macro_rules! ord_with_self {
    ( $( $buf_type:ty ),* ) => {
        $(
        impl PartialEq for $buf_type {
            fn eq(&self, other: &Self) -> bool {
                self.data == other.data
            }
        }

        impl PartialOrd for $buf_type {
            fn partial_cmp(&self, other: &Self)  -> Option<Ordering> {
                self.data.partial_cmp(&other.data)
            }
        }
        )*
    }
}

macro_rules! ord_with_field {
    ( $( $buf_type:ty { $field:ty } ),* ) =>  {
        $(
        impl PartialEq<$field> for $buf_type {
            fn eq(&self, other: &$field) -> bool {
                self.data == *other
            }
        }

        impl PartialOrd<$field> for $buf_type {
            fn partial_cmp(&self, other: &$field) -> Option<Ordering> {
                self.data.partial_cmp(other)
            }
        }
        )*
    }
}

#[derive(Debug, Clone)]
pub struct VarInt {
    data: i32,
    buffer: Vec<u8>
}

impl VarInt {
    fn change(&mut self, t: i32) {
        self.data = t;
        let mut value: u32 = {
            let bytes = t.to_be_bytes();
            u32::from_be_bytes(bytes)
        };
        loop {
            if (value & !0x7F) == 0 {
                self.buffer.push(value as u8);
                return;
            }

            self.buffer.push(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }
}

impl From<i32> for VarInt {
    fn from(t: i32) -> Self {
        let mut buffer = Vec::new();
        let mut value: u32 = {
            let bytes = t.to_be_bytes();
            u32::from_be_bytes(bytes)
        };
        loop {
            if (value & !0x7F) == 0 {
                buffer.push(value as u8);
                return VarInt {
                    data: t,
                    buffer: buffer
                };
            }

            buffer.push(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }
}

impl From<VarInt> for i32 {
    fn from(t: VarInt) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct VarLong {
    data: i64,
    buffer: Vec<u8>
}

impl VarLong {
    fn change(&mut self, t: i64) {
        self.data = t;
        let mut value: u64 = {
            let bytes = t.to_be_bytes();
            u64::from_be_bytes(bytes)
        };
        loop {
            if value & !(0x7F as u64) == 0 {
                self.buffer.push(value as u8);
                return;
            }

            self.buffer.push(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }
}

impl From<i64> for VarLong {
    fn from(t: i64) -> Self {
        let mut buffer: Vec<u8> = Vec::new();
        let mut value: u64 = {
            let bytes = t.to_be_bytes();
            u64::from_be_bytes(bytes)
        };
        loop {
            if value & !(0x7F as u64) == 0 {
                buffer.push(value as u8);
                return VarLong {
                    data: t,
                    buffer: buffer
                };
            }

            buffer.push(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }
}

impl From<VarLong> for i64 {
    fn from(t: VarLong) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct Short {
    data: i16,
    buffer: Vec<u8>
}

impl Short {
    fn change(&mut self, t: i16) {
        self.data = t;
        let value: u16 = {
            let bytes = t.to_be_bytes();
            u16::from_be_bytes(bytes)
        };
        for i in 0..2 {
            self.buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
    }
}

impl From<i16> for Short {
    fn from(t: i16) -> Short {
        let mut buffer: Vec<u8> = Vec::new();
        let value: u16 = {
            let bytes = t.to_be_bytes();
            u16::from_be_bytes(bytes)
        };
        for i in 0..2 {
            buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
        Short {
            data: t,
            buffer
        }
    }
}

impl From<Short> for i16 {
    fn from(t: Short) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct Int {
    data: i32,
    buffer: Vec<u8>
}

impl Int {
    fn change(&mut self, t: i32) {
        self.data = t;
        let value: u32 = {
            let bytes = t.to_be_bytes();
            u32::from_be_bytes(bytes)
        };
        for i in 0..4 {
            self.buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
    }
}

impl From<i32> for Int {
    fn from(t: i32) -> Int {
        let mut buffer: Vec<u8> = Vec::new();
        let value: u32 = {
            let bytes = t.to_be_bytes();
            u32::from_be_bytes(bytes)
        };
        for i in 0..4 {
            buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
        Int {
            data: t,
            buffer
        }
    }
}

impl From<Int> for i32 {
    fn from(t: Int) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct Long {
    data: i64,
    buffer: Vec<u8>
}

impl Long {
    fn change(&mut self, t: i64) {
        self.data = t;
        let value: u64 = {
            let bytes = t.to_be_bytes();
            u64::from_be_bytes(bytes)
        };
        for i in 0..8 {
            self.buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
    }
}

impl From<i64> for Long {
    fn from(t: i64) -> Long {
        let mut buffer: Vec<u8> = Vec::new();
        let value: u64 = {
            let bytes = t.to_be_bytes();
            u64::from_be_bytes(bytes)
        };
        for i in 0..8 {
            buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
        }
        Long {
            data: t,
            buffer
        }
    }
}

impl From<Long> for i64 {
    fn from(t: Long) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    data: f32,
    buffer: Vec<u8>
}

impl Float {
    fn change(&mut self, t: f32) {
        self.data = t;
        unsafe {
            let value: u32 = std::mem::transmute::<f32, u32>(t);
            for i in 0..4 {
                self.buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
            }
        }
    }
}

impl From<f32> for Float {
    fn from(t: f32) -> Float {
        unsafe {
            let mut buffer: Vec<u8> = Vec::new();
            let value: u32 = std::mem::transmute::<f32, u32>(t);
            for i in 0..4 {
                buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
            }
            Float {
                data: t,
                buffer
            }
        }
    }
}

impl From<Float> for f32 {
    fn from(t: Float) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct Double {
    data: f64,
    buffer: Vec<u8>
}

impl Double {
    fn change(&mut self, t: f64) {
        self.data = t;
        unsafe {
            let value: u64 = std::mem::transmute::<f64, u64>(t);
            for i in 0..8 {
                self.buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
            }
        }
    }
}

impl From<f64> for Double{
    fn from(t: f64) -> Double {
        unsafe {
            let mut buffer: Vec<u8> = Vec::new();
            let value: u64 = std::mem::transmute::<f64, u64>(t);
            for i in 0..8 {
                buffer.push(((value >> (i * 8)) & 0xFF).try_into().unwrap());
            }
            Double {
                data: t,
                buffer
            }
        }
    }
}

impl From<Double> for f64 {
    fn from(t: Double) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct Str {
    data: String,
    buffer: Vec<u8>
}

impl Str {
    fn change(&mut self, t: String) {
        self.data = t.clone();
        self.buffer.clear();
        self.buffer.append(&mut VarInt::from(t.len() as i32).buffer);
        self.buffer.append(&mut t.as_bytes().to_vec());
    }
}

impl From<String> for Str {
    fn from(t: String) -> Str {
        let mut buffer: Vec<u8> =  Vec::new();
        buffer.append(&mut VarInt::from(t.len() as i32).buffer);
        buffer.append(&mut t.as_bytes().to_vec());
        Str {
            data: t,
            buffer
        }
    }
}

impl From<Str> for String {
    fn from(t: Str) -> Self {
        t.data
    }
}

#[derive(Debug, Clone)]
pub struct ByteArray {
    buffer: Vec<u8>
}

impl ByteArray {
    fn change(&mut self, t: Vec<u8>) {
        self.buffer = t;
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(t: Vec<u8>) -> ByteArray {
        ByteArray {
            buffer: t
        }
    }
}

impl From<ByteArray> for Vec<u8> {
    fn from(t: ByteArray) -> Self {
        t.buffer
    }
}

#[derive(Debug, Clone)]
pub struct Bool {
    data: bool,
    buffer: Vec<u8>
}

impl Bool {
    fn change(&mut self, t: bool) {
        self.data = t;
        self.buffer = vec![if t {0x01} else {0x00}];
    }
}

impl From<bool> for Bool {
    fn from(t: bool) -> Bool {
        let buffer = vec![if t {0x01} else {0x00}];
        Bool {
            data: t,
            buffer
        }
    }
}

impl From<Bool> for bool {
    fn from(t: Bool) -> bool {
        t.data
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Byte {
    data: u8
}

impl Byte {
    fn change(&mut self, t: u8) {
        self.data = t;
    }
}

impl From<u8> for Byte {
    fn from(t: u8) -> Byte {
        Byte {
            data: t
        }
    }
}

impl From<Byte> for u8 {
    fn from(t: Byte) -> u8 {
        t.data
    }
}

ord_with_self!(VarInt, VarLong, Short, Int, Long, Float, Double, Bool, Byte);

ord_with_field!(VarInt { i32 }, VarLong { i64 }, Short { i16 }, Int { i32 }, Long { i64 },
                Float { f32 }, Double { f64 }, Bool { bool }, Byte { u8 });


#[derive(Clone)]
pub enum BufTypes {
    VarInt(VarInt),
    VarLong(VarLong),
    Short(Short),
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Str(Str),
    ByteArray(ByteArray),
    Bool(Bool),
    Byte(Byte),
    Fail
}
