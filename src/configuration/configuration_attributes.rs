use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConfigurationAttributes {
    pub self_powered: bool,
    pub remote_wakeup: bool,
}

impl EncodeByte for ConfigurationAttributes {
    fn encode(&self) -> Result<u8, &str> {
        // D7 Reserved, set to 1. (USB 1.0 Bus Powered)
        // D6 Self Powered
        // D5 Remote Wakeup
        // D4..0 Reserved, set to 0.
        let mut byte = 0b1000_0000;
        byte |= (self.self_powered as u8) << 6;
        byte |= (self.remote_wakeup as u8) << 5;
        Ok(byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let attributes = ConfigurationAttributes {
            self_powered: true,
            remote_wakeup: false,
        };
        assert_eq!(attributes.encode().unwrap(), 0b1100_0000);
        
        let attributes = ConfigurationAttributes {
            self_powered: false,
            remote_wakeup: true,
        };
        
        assert_eq!(attributes.encode().unwrap(), 0b1010_0000);
        
        let attributes = ConfigurationAttributes {
            self_powered: true,
            remote_wakeup: true,
        };
        
        assert_eq!(attributes.encode().unwrap(), 0b1110_0000);
    }
}
