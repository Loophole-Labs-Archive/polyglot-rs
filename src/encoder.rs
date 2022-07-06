use std::io::{Cursor, Write};
use byteorder::{BigEndian, WriteBytesExt};
use crate::kind::Kind;

pub trait Encoder {
    fn encode_none(self) -> Self;
    fn encode_array(self, size: usize, val_kind: Kind) -> Self;
    fn encode_map(self, size: usize, key_kind: Kind, val_kind: Kind) -> Self;
    fn encode_bytes(self, val: &[u8]) -> Self;
    fn encode_string(self, val: &str) -> Self;
    fn encode_error(self, val: &str) -> Self;
    fn encode_bool(self, val: bool) -> Self;
    fn encode_u8(self, val: u8) -> Self;
    fn encode_u16(self, val: u16) -> Self;
    fn encode_u32(self, val: u32) -> Self;
    fn encode_u64(self, val: u64) -> Self;
    fn encode_i32(self, val: i32) -> Self;
    fn encode_i64(self, val: i64) -> Self;
    fn encode_f32(self, val: f32) -> Self;
    fn encode_f64(self, val: f64) -> Self;
}

impl Encoder for Cursor<Vec<u8>> {
    #[must_use]
    fn encode_none(mut self) -> Self {
        self.write_u8(Kind::None as u8).unwrap();
        self
    }

    #[must_use]
    fn encode_array(mut self, size: usize, val_kind: Kind) -> Self {
        self.write_u8(Kind::Array as u8).unwrap();
        self.write_u8(val_kind as u8).unwrap();
        self.encode_u32(size as u32)
    }

    #[must_use]
    fn encode_map(mut self, size: usize, key_kind: Kind, val_kind: Kind) -> Self {
        self.write_u8(Kind::Map as u8).unwrap();
        self.write_u8(key_kind as u8).unwrap();
        self.write_u8(val_kind as u8).unwrap();
        self = self.encode_u32(size as u32);
        self
    }

    #[must_use]
    fn encode_bytes(mut self, val: &[u8]) -> Self {
        self.write_u8(Kind::Bytes as u8).unwrap();
        self = self.encode_u32(val.len() as u32);
        self.write_all(val).unwrap();
        self
    }

    #[must_use]
    fn encode_string(mut self, val: &str) -> Self {
        let b = val.as_bytes();
        self.write_u8(Kind::String as u8).unwrap();
        self = self.encode_u32(b.len() as u32);
        self.write_all(b).unwrap();
        self
    }

    #[must_use]
    fn encode_error(mut self, val: &str) -> Self {
        let b = val.as_bytes();
        self.write_u8(Kind::Error as u8).unwrap();
        self = self.encode_u32(b.len() as u32);
        self.write_all(b).unwrap();
        self
    }

    #[must_use]
    fn encode_bool(mut self, val: bool) -> Self {
        self.write_u8(Kind::Bool as u8).unwrap();
        self.write_u8(val as u8).unwrap();
        self
    }

    #[must_use]
    fn encode_u8(mut self, val: u8) -> Self {
        self.write_u8(Kind::U8 as u8).unwrap();
        self.write_u8(val).unwrap();
        self
    }

    #[must_use]
    fn encode_u16(mut self, val: u16) -> Self {
        self.write_u8(Kind::U16 as u8).unwrap();
        self.write_u16::<BigEndian>(val).unwrap();
        self
    }

    #[must_use]
    fn encode_u32(mut self, val: u32) -> Self {
        self.write_u8(Kind::U32 as u8).unwrap();
        self.write_u32::<BigEndian>(val).unwrap();
        self
    }

    #[must_use]
    fn encode_u64(mut self, val: u64) -> Self {
        self.write_u8(Kind::U64 as u8).unwrap();
        self.write_u64::<BigEndian>(val).unwrap();
        self
    }

    #[must_use]
    fn encode_i32(mut self, val: i32) -> Self {
        self.write_u8(Kind::I32 as u8).unwrap();
        self.write_i32::<BigEndian>(val).unwrap();
        self
    }

    #[must_use]
    fn encode_i64(mut self, val: i64) -> Self {
        self.write_u8(Kind::I64 as u8).unwrap();
        self.write_i64::<BigEndian>(val).unwrap();
        self
    }

    #[must_use]
    fn encode_f32(mut self, val: f32) -> Self {
        self.write_u8(Kind::F32 as u8).unwrap();
        self.write_f32::<BigEndian>(val).unwrap();
        self
    }

    #[must_use]
    fn encode_f64(mut self, val: f64) -> Self {
        self.write_u8(Kind::F64 as u8).unwrap();
        self.write_f64::<BigEndian>(val).unwrap();
        self
    }
}