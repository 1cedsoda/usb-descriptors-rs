use core::fmt::{Display, Formatter};

use alloc::vec::Vec;

use crate::binary::EncodeBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageCode(u16);

impl EncodeBytes for LanguageCode {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        Ok(self.0.to_le_bytes().to_vec())
    }
}

impl Display for LanguageCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

pub const EN_US: LanguageCode = LanguageCode(0x0409); // English - United States
pub const DE_DE: LanguageCode = LanguageCode(0x0407); // German - Germany

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code_encode() {
        assert_eq!(EN_US.encode().unwrap(), vec![0x09, 0x04]);
        assert_eq!(DE_DE.encode().unwrap(), vec![0x07, 0x04]);
    }
}
