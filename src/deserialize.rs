use rust_decimal::Decimal;
use std::io::Read;
use uuid::Uuid;

pub trait Deserialize: Sized {
    fn deserialize(src: &mut &[u8]) -> std::io::Result<Self>;
}

impl Deserialize for i64 {
    fn deserialize(src: &mut &[u8]) -> std::io::Result<Self> {
        let mut buf = [0; 8];
        src.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for u64 {
    fn deserialize(src: &mut &[u8]) -> std::io::Result<Self> {
        let mut buf = [0; 8];
        src.read_exact(&mut buf)?;
        Ok(Self::from_le_bytes(buf))
    }
}

impl Deserialize for Uuid {
    fn deserialize(src: &mut &[u8]) -> std::io::Result<Self> {
        let mut buf = [0; 16];
        src.read_exact(&mut buf)?;
        Ok(Self::from_bytes_le(buf))
    }
}

impl Deserialize for Decimal {
    fn deserialize(src: &mut &[u8]) -> std::io::Result<Self> {
        let mut buf = [0; 16];
        src.read_exact(&mut buf)?;
        Ok(Self::deserialize(buf))
    }
}
