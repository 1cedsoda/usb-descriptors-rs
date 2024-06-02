use alloc::vec::Vec;

use crate::binary::EncodeBytes;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConfigurationAttributes {
    pub self_powered: bool,
    pub remote_wakeup: bool,
}

impl EncodeBytes for ConfigurationAttributes {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push((self.self_powered as u8) << 6 | (self.remote_wakeup as u8) << 5);
        Ok(bytes)
    }
}
