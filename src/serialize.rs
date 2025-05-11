use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use uuid::Uuid;

pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

impl Serialize for i64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl Serialize for u64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
}

impl Serialize for Uuid {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_bytes_le());
    }
}

impl Serialize for Decimal {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_f64().unwrap().to_le_bytes());
    }
}
