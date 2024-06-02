use alloc::vec::Vec;

use crate::binary::EncodeBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BcdVersion {
    pub major: u8,
    pub minor: u8,
}

impl EncodeBytes for BcdVersion {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.minor);
        bytes.push(self.major);
        Ok(bytes)
    }
}

pub const USB_1_0: BcdVersion = BcdVersion {
    major: 0x01,
    minor: 0x00,
};
pub const USB_1_1: BcdVersion = BcdVersion {
    major: 0x01,
    minor: 0x01,
};
pub const USB_2_0: BcdVersion = BcdVersion {
    major: 0x02,
    minor: 0x00,
};
pub const USB_2_1: BcdVersion = BcdVersion {
    major: 0x02,
    minor: 0x01,
};
pub const USB_3_0: BcdVersion = BcdVersion {
    major: 0x03,
    minor: 0x00,
};
pub const USB_3_1: BcdVersion = BcdVersion {
    major: 0x03,
    minor: 0x01,
};
pub const USB_3_2: BcdVersion = BcdVersion {
    major: 0x03,
    minor: 0x02,
};
