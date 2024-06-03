use core::fmt::Display;

use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DescriptorType {
    Device = 0x01,
    Configuration = 0x02,
    String = 0x03,
    Interface = 0x04,
    Endpoint = 0x05,
}

impl Display for DescriptorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DescriptorType::Device => write!(f, "Device"),
            DescriptorType::Configuration => write!(f, "Configuration"),
            DescriptorType::String => write!(f, "String"),
            DescriptorType::Interface => write!(f, "Interface"),
            DescriptorType::Endpoint => write!(f, "Endpoint"),
        }
    }
}

impl EncodeByte for DescriptorType {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}

impl DescriptorType {
    pub fn from_value(value: u16) -> Result<Self, &'static str> {
        match value {
            0x0100 => Ok(DescriptorType::Device),
            0x0200 => Ok(DescriptorType::Configuration),
            0x0400 => Ok(DescriptorType::Interface),
            0x0500 => Ok(DescriptorType::Endpoint),
            _ => Err("Invalid descriptor type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DESCRIPTOR_TYPE_DEVICE: u8 = 1;
    const DESCRIPTOR_TYPE_CONFIGURATION: u8 = 2;
    const DESCRIPTOR_TYPE_INTERFACE: u8 = 4;
    const DESCRIPTOR_TYPE_ENDPOINT: u8 = 5;

    #[test]
    fn test_encode() {
        let descriptor_type_device = DescriptorType::Device.encode().unwrap();
        assert_eq!(descriptor_type_device, DESCRIPTOR_TYPE_DEVICE);

        let descriptor_type_configuration = DescriptorType::Configuration.encode().unwrap();
        assert_eq!(descriptor_type_configuration, DESCRIPTOR_TYPE_CONFIGURATION);

        let descriptor_type_interface = DescriptorType::Interface.encode().unwrap();
        assert_eq!(descriptor_type_interface, DESCRIPTOR_TYPE_INTERFACE);

        let descriptor_type_endpoint = DescriptorType::Endpoint.encode().unwrap();
        assert_eq!(descriptor_type_endpoint, DESCRIPTOR_TYPE_ENDPOINT);
    }
}
