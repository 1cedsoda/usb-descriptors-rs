use alloc::vec::Vec;

use crate::binary::EncodeBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageCode(u16);

impl EncodeBytes for LanguageCode {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

pub const EN_US: LanguageCode = LanguageCode(0x0409);
pub const DE_DE: LanguageCode = LanguageCode(0x0407);
