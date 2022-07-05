use crate::kind::Kind;

#[inline(always)]
pub(crate) fn encode_none(buf: &mut Vec<u8>) {
    buf.push(Kind::None as u8);
}

#[inline(always)]
pub(crate) fn encode_array(buf: &mut Vec<u8>, size: u32, val_kind: Kind) {
    buf.push(Kind::Array as u8);
    buf.push(val_kind as u8);
    encode_u32(buf, size);
}

#[inline(always)]
pub(crate) fn encode_map(buf: &mut Vec<u8>, size: u32, key_kind: Kind, val_kind: Kind) {
    buf.push(Kind::Map as u8);
    buf.push(key_kind as u8);
    buf.push(val_kind as u8);
    encode_u32(buf, size);
}

#[inline(always)]
pub(crate) fn encode_bytes(buf: &mut Vec<u8>, val: &[u8]) {
    buf.push(Kind::Bytes as u8);
    encode_u32(buf, val.len() as u32);
    buf.extend_from_slice(val);
}

#[inline(always)]
pub(crate) fn encode_string(buf: &mut Vec<u8>, val: &str) {
    let b = val.as_bytes();
    buf.push(Kind::String as u8);
    encode_u32(buf, b.len() as u32);
    buf.extend_from_slice(b);
}

#[inline(always)]
pub(crate) fn encode_error(buf: &mut Vec<u8>, val: &str) {
    let b = val.to_string().as_bytes().to_owned();
    buf.push(Kind::Error as u8);
    encode_u32(buf, b.len() as u32);
    buf.extend_from_slice(b.as_slice());
}

#[inline(always)]
pub(crate) fn encode_bool(buf: &mut Vec<u8>, val: bool) {
    buf.push(Kind::Bool as u8);
    buf.push(val as u8);
}

#[inline(always)]
pub(crate) fn encode_u8(buf: &mut Vec<u8>, val: u8) {
    buf.push(Kind::U8 as u8);
    buf.push(val);
}

#[inline(always)]
pub(crate) fn encode_u16(buf: &mut Vec<u8>, val: u16) {
    buf.push(Kind::U16 as u8);
    buf.push(((val >> 8) & 0xff) as u8);
    buf.push((val & 0xff) as u8);
}

#[inline(always)]
pub(crate) fn encode_u32(buf: &mut Vec<u8>, val: u32) {
    buf.push(Kind::U32 as u8);
    buf.push(((val >> 24) & 0xff) as u8);
    buf.push(((val >> 16) & 0xff) as u8);
    buf.push(((val >> 8) & 0xff) as u8);
    buf.push((val & 0xff) as u8);
}

#[inline(always)]
pub(crate) fn encode_u64(buf: &mut Vec<u8>, val: u64) {
    buf.push(Kind::U64 as u8);
    buf.push(((val >> 56) & 0xff) as u8);
    buf.push(((val >> 48) & 0xff) as u8);
    buf.push(((val >> 40) & 0xff) as u8);
    buf.push(((val >> 32) & 0xff) as u8);
    buf.push(((val >> 24) & 0xff) as u8);
    buf.push(((val >> 16) & 0xff) as u8);
    buf.push(((val >> 8) & 0xff) as u8);
    buf.push((val & 0xff) as u8);
}

#[inline(always)]
pub(crate) fn encode_i32(buf: &mut Vec<u8>, val: i32) {
    let cast_val = val as u32;
    buf.push(Kind::I32 as u8);
    buf.push(((cast_val >> 24) & 0xff) as u8);
    buf.push(((cast_val >> 16) & 0xff) as u8);
    buf.push(((cast_val >> 8) & 0xff) as u8);
    buf.push((cast_val & 0xff) as u8);
}

#[inline(always)]
pub(crate) fn encode_i64(buf: &mut Vec<u8>, val: i64) {
    let cast_val = val as u64;
    buf.push(Kind::I64 as u8);
    buf.push(((cast_val >> 56) & 0xff) as u8);
    buf.push(((cast_val >> 48) & 0xff) as u8);
    buf.push(((cast_val >> 40) & 0xff) as u8);
    buf.push(((cast_val >> 32) & 0xff) as u8);
    buf.push(((cast_val >> 24) & 0xff) as u8);
    buf.push(((cast_val >> 16) & 0xff) as u8);
    buf.push(((cast_val >> 8) & 0xff) as u8);
    buf.push((cast_val & 0xff) as u8);
}

#[inline(always)]
pub(crate) fn encode_f32(buf: &mut Vec<u8>, val: f32) {
    let cast_val = val.to_bits();
    buf.push(Kind::F32 as u8);
    buf.push(((cast_val >> 24) & 0xff) as u8);
    buf.push(((cast_val >> 16) & 0xff) as u8);
    buf.push(((cast_val >> 8) & 0xff) as u8);
    buf.push((cast_val & 0xff) as u8);
}

#[inline(always)]
pub(crate) fn encode_f64(buf: &mut Vec<u8>, val: f64) {
    let cast_val = val.to_bits();
    buf.push(Kind::I64 as u8);
    buf.push(((cast_val >> 56) & 0xff) as u8);
    buf.push(((cast_val >> 48) & 0xff) as u8);
    buf.push(((cast_val >> 40) & 0xff) as u8);
    buf.push(((cast_val >> 32) & 0xff) as u8);
    buf.push(((cast_val >> 24) & 0xff) as u8);
    buf.push(((cast_val >> 16) & 0xff) as u8);
    buf.push(((cast_val >> 8) & 0xff) as u8);
    buf.push((cast_val & 0xff) as u8);
}