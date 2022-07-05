use crate::encode;
use crate::kind::Kind;

pub trait Encoder {
    fn encode_none(self) -> Self;
    fn encode_array(self, size: u32, val_kind: Kind) -> Self;
    fn encode_map(self, size: u32, key_kind: Kind, val_kind: Kind) -> Self;
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

impl Encoder for Vec<u8> {
    fn encode_none(mut self) -> Self {
        encode::encode_none(&mut self);
        return self
    }

    fn encode_array(mut self, size: u32, val_kind: Kind) -> Self {
        encode::encode_array(&mut self, size, val_kind);
        return self
    }

    fn encode_map(mut self, size: u32, key_kind: Kind, val_kind: Kind) -> Self {
        encode::encode_map(&mut self, size, key_kind, val_kind);
        return self
    }

    fn encode_bytes(mut self, val: &[u8]) -> Self {
        encode::encode_bytes(&mut self, val);
        return self
    }

    fn encode_string(mut self, val: &str) -> Self {
        encode::encode_string( &mut self, val);
        self
    }

    fn encode_error(mut self, val: &str) -> Self {
        encode::encode_error(&mut self, val);
        self
    }

    fn encode_bool(mut self, val: bool) -> Self {
        encode::encode_bool(&mut self, val);
        self
    }

    fn encode_u8(mut self, val: u8) -> Self {
        encode::encode_u8(&mut self, val);
        self
    }

    fn encode_u16(mut self, val: u16) -> Self {
        encode::encode_u16(&mut self, val);
        self
    }

    fn encode_u32(mut self, val: u32) -> Self {
        encode::encode_u32(&mut self, val);
        self
    }

    fn encode_u64(mut self, val: u64) -> Self {
        encode::encode_u64(&mut self, val);
        self
    }

    fn encode_i32(mut self, val: i32) -> Self {
        encode::encode_i32(&mut self, val);
        self
    }

    fn encode_i64(mut self, val: i64) -> Self {
        encode::encode_i64(&mut self, val);
        self
    }

    fn encode_f32(mut self, val: f32) -> Self {
        encode::encode_f32(&mut self, val);
        self
    }

    fn encode_f64(mut self, val: f64) -> Self {
        encode::encode_f64(&mut self, val);
        self
    }
}