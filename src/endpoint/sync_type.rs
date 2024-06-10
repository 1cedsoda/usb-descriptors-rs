use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SyncType {
    NoSync,
    Asynchronous,
    Adaptive,
    Synchronous,
}

impl EncodeByte for SyncType {
    fn encode(&self) -> Result<u8, &str> {
        match *self {
            SyncType::NoSync => Ok(0x00),
            SyncType::Asynchronous => Ok(0x01),
            SyncType::Adaptive => Ok(0x02),
            SyncType::Synchronous => Ok(0x03),
        }
    }
}
