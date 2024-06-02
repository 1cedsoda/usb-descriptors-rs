use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TransferType {
    Control = 0x00,
    Isochronous = 0x01,
    Bulk = 0x02,
    Interrupt = 0x03,
}
impl EncodeByte for TransferType {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}
