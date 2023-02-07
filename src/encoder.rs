/*
    Copyright 2022 Loophole Labs

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

           http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
bv c */

use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::kind::Kind;
use byteorder::{BigEndian, WriteBytesExt};
use std::io;
use std::io::{Cursor, Write};

const CONTINUATION: u8 = 0x80;

#[derive(Debug, PartialEq)]
pub enum EncodingError {
    WriteFailed
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for EncodingError {}

impl From<io::Error> for EncodingError {
    fn from(_: io::Error) -> Self {
        EncodingError::WriteFailed
    }
}

pub trait Encoder {
    fn encode_none(self) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_array(self, size: usize, val_kind: Kind) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_map(self, size: usize, key_kind: Kind, val_kind: Kind) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_bytes(self, val: &[u8]) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_string(self, val: &str) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_error(self, val: Box<dyn std::error::Error>) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_bool(self, val: bool) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_u8(self, val: u8) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_u16(self, val: u16) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_u32(self, val: u32) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_u64(self, val: u64) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_i32(self, val: i32) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_i64(self, val: i64) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_f32(self, val: f32) -> Result<Self, EncodingError>
    where
        Self: Sized;
    fn encode_f64(self, val: f64) -> Result<Self, EncodingError>
    where
        Self: Sized;
}

impl Encoder for &mut Cursor<Vec<u8>> {
    fn encode_none(self) -> Result<Self, EncodingError> {
        self.write_u8(Kind::None as u8).unwrap();
        Ok(self)
    }

    fn encode_array(self, size: usize, val_kind: Kind) -> Result<Self, EncodingError> {
        self.write_u8(Kind::Array as u8)?;
        self.write_u8(val_kind as u8)?;
        self.encode_u32(size as u32)
    }

    fn encode_map(
        self,
        size: usize,
        key_kind: Kind,
        val_kind: Kind,
    ) -> Result<Self, EncodingError> {
        self.write_u8(Kind::Map as u8)?;
        self.write_u8(key_kind as u8)?;
        self.write_u8(val_kind as u8)?;
        self.encode_u32(size as u32)
    }

    fn encode_bytes(self, val: &[u8]) -> Result<Self, EncodingError> {
        self.write_u8(Kind::Bytes as u8)?;
        self.encode_u32(val.len() as u32)?;
        self.write_all(val)?;
        Ok(self)
    }

    fn encode_string(self, val: &str) -> Result<Self, EncodingError> {
        let b = val.as_bytes();
        self.write_u8(Kind::String as u8)?;
        self.encode_u32(b.len() as u32)?;
        self.write_all(b).ok().ok_or(EncodingError::WriteFailed)?;
        Ok(self)
    }

    fn encode_error(self, val: Box<dyn std::error::Error>) -> Result<Self, EncodingError> {
        let b = val.to_string().as_bytes().to_owned();
        self.write_u8(Kind::Error as u8)?;
        self.write_u8(Kind::String as u8)?;
        self.encode_u32(b.len() as u32)?;
        self.write_all(b.as_slice())
            .ok()
            .ok_or(EncodingError::WriteFailed)?;
        Ok(self)
    }

    fn encode_bool(self, val: bool) -> Result<Self, EncodingError> {
        self.write_u8(Kind::Bool as u8)?;
        self.write_u8(val as u8)?;
        Ok(self)
    }

    fn encode_u8(self, val: u8) -> Result<Self, EncodingError> {
        self.write_u8(Kind::U8 as u8)?;
        self.write_u8(val)?;
        Ok(self)
    }

    fn encode_u16(self, val: u16) -> Result<Self, EncodingError> {
        let mut val = val;
        self.write_u8(Kind::U16 as u8)?;
        while val >= CONTINUATION as u16 {
            self.write_u8(val as u8 | CONTINUATION)?;
            val >>= 7;
        }
        self.write_u8(val as u8)?;
        Ok(self)
    }

    fn encode_u32(self, val: u32) -> Result<Self, EncodingError> {
        let mut val = val;
        self.write_u8(Kind::U32 as u8)?;
        while val >= CONTINUATION as u32 {
            self.write_u8(val as u8 | CONTINUATION)?;
            val >>= 7;
        }
        self.write_u8(val as u8)?;
        Ok(self)
    }

    fn encode_u64(self, val: u64) -> Result<Self, EncodingError> {
        let mut val = val;
        self.write_u8(Kind::U64 as u8)?;
        while val >= CONTINUATION as u64 {
            self.write_u8(val as u8 | CONTINUATION)?;
            val >>= 7;
        }
        self.write_u8(val as u8)?;
        Ok(self)
    }

    fn encode_i32(self, val: i32) -> Result<Self, EncodingError> {
        self.write_u8(Kind::I32 as u8)?;

        // Shift the value to the left by 1 bit, then flip the bits if the value is negative.
        let mut cast_val = (val as u32) << 1;
        if val < 0 {
            cast_val = !cast_val;
        }

        while cast_val >= CONTINUATION as u32 {
            // Append the lower 7 bits of the value, then shift the value to the right by 7 bits.
            self.write_u8(cast_val as u8 | CONTINUATION)?;
            cast_val >>= 7;
        }
        self.write_u8(cast_val as u8)?;
        Ok(self)
    }

    fn encode_i64(self, val: i64) -> Result<Self, EncodingError> {
        self.write_u8(Kind::I64 as u8)?;

        // Shift the value to the left by 1 bit, then flip the bits if the value is negative.
        let mut cast_val = (val as u64) << 1;
        if val < 0 {
            cast_val = !cast_val;
        }

        while cast_val >= CONTINUATION as u64 {
            // Append the lower 7 bits of the value, then shift the value to the right by 7 bits.
            self.write_u8(cast_val as u8 | CONTINUATION)?;
            cast_val >>= 7;
        }
        self.write_u8(cast_val as u8)?;
        Ok(self)
    }

    fn encode_f32(self, val: f32) -> Result<Self, EncodingError> {
        self.write_u8(Kind::F32 as u8)?;
        self.write_f32::<BigEndian>(val)?;
        Ok(self)
    }

    fn encode_f64(self, val: f64) -> Result<Self, EncodingError> {
        self.write_u8(Kind::F64 as u8)?;
        self.write_f64::<BigEndian>(val)?;
        Ok(self)
    }
}
