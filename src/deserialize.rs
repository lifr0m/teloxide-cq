use rust_decimal::Decimal;
use std::io::{Cursor, Read};
use std::string::FromUtf8Error;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("string decode: {0}")]
    StringDecode(#[from] FromUtf8Error),
}

pub trait Deserialize: Sized {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error>;
}

impl Deserialize for u8 {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 1];
        src.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for i64 {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 8];
        src.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for u64 {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 8];
        src.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for Uuid {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 16];
        src.read_exact(&mut buf)?;
        Ok(Self::from_bytes_le(buf))
    }
}

impl Deserialize for Decimal {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buf = [0; 16];
        src.read_exact(&mut buf)?;
        Ok(Self::deserialize(buf))
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let len = u8::deserialize(src)? as usize;
        (0..len).map(|_| T::deserialize(src)).collect()
    }
}

impl Deserialize for String {
    fn deserialize(src: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let buf = Vec::<u8>::deserialize(src)?;
        Ok(String::from_utf8(buf)?)
    }
}
