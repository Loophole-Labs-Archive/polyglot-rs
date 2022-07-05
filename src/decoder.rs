use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};
use std::str;
use crate::kind::Kind;

pub enum DecodingError {
    InvalidU32,
    InvalidString
}

pub trait Decoder {
    fn decode_u32(&mut self) -> Result<u32, DecodingError>;
    fn decode_string(&mut self) -> Result<String, DecodingError>;
}

impl Decoder for Cursor<Vec<u8>> {
    fn decode_u32(&mut self) -> Result<u32, DecodingError> {
        let kind = self.read_u8().ok().ok_or(DecodingError::InvalidU32)?;
        if kind == Kind::U32 as u8 {
            return self.read_u32::<BigEndian>().ok().ok_or(DecodingError::InvalidU32);
        } else {
            Err(DecodingError::InvalidU32)
        }
    }

    fn decode_string(&mut self) -> Result<String, DecodingError> {
        let kind = self.read_u8().ok().ok_or(DecodingError::InvalidString)?;
        if kind == Kind::String as u8 {
            let size= self.decode_u32()? as usize;
            let mut str_buf = vec![0u8; size];
            self.read_exact(&mut str_buf).ok().ok_or(DecodingError::InvalidString)?;

            let result = str::from_utf8(&*str_buf)
                .ok()
                .ok_or(DecodingError::InvalidString)?;
            Ok(result.to_owned())
        } else {
            Err(DecodingError::InvalidString)
        }
    }
}