use rust_decimal::Decimal;
use uuid::Uuid;

pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

impl Serialize for u8 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.extend(self.to_le_bytes());
    }
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
        buf.extend(self.serialize());
    }
}

impl<T: Serialize> Serialize for [T] {
    fn serialize(&self, buf: &mut Vec<u8>) {
        (self.len() as u8).serialize(buf);
        self.iter().for_each(|e| e.serialize(buf));
    }
}

impl Serialize for str {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_bytes().serialize(buf);
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_str().serialize(buf);
    }
}
