use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TransferType {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

impl EncodeByte for TransferType {
    fn encode(&self) -> Result<u8, &str> {
        match *self {
            TransferType::Control => Ok(0x00),
            TransferType::Isochronous => Ok(0x01),
            TransferType::Bulk => Ok(0x02),
            TransferType::Interrupt => Ok(0x03),
        }
    }
}
