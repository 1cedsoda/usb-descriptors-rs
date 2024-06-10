use alloc::vec::Vec;

use crate::binary::EncodeBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl EncodeBytes for Version {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(self.minor);
        bytes.push(self.major);
        Ok(bytes)
    }
}

pub const USB1_0: Version = Version {
    major: 0x01,
    minor: 0x00,
};
pub const USB1_1: Version = Version {
    major: 0x01,
    minor: 0x01,
};
pub const USB2_0: Version = Version {
    major: 0x02,
    minor: 0x00,
};
pub const USB2_1: Version = Version {
    major: 0x02,
    minor: 0x01,
};
pub const USB3_0: Version = Version {
    major: 0x03,
    minor: 0x00,
};
pub const USB3_1: Version = Version {
    major: 0x03,
    minor: 0x01,
};
pub const USB3_2: Version = Version {
    major: 0x03,
    minor: 0x02,
};
