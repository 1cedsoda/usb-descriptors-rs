use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SyncType {
    NoSync = 0x00,
    Asynchronous = 0x01,
    Adaptive = 0x02,
    Synchronous = 0x03,
}
impl EncodeByte for SyncType {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}
