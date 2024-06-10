use core::fmt::Display;

use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DescriptorType {
    Device,
    Configuration,
    String,
    Interface,
    Endpoint,
    Hid,
    Report,
}

impl DescriptorType {
    pub fn from_value(value: u16) -> Result<Self, &'static str> {
        match value {
            0x0100 => Ok(DescriptorType::Device),
            0x0200 => Ok(DescriptorType::Configuration),
            0x0400 => Ok(DescriptorType::Interface),
            0x0500 => Ok(DescriptorType::Endpoint),
            0x2100 => Ok(DescriptorType::Hid),
            0x2200 => Ok(DescriptorType::Report),
            _ => Err("Invalid descriptor type"),
        }
    }
}

impl Display for DescriptorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DescriptorType::Device => write!(f, "Device"),
            DescriptorType::Configuration => write!(f, "Configuration"),
            DescriptorType::String => write!(f, "String"),
            DescriptorType::Interface => write!(f, "Interface"),
            DescriptorType::Endpoint => write!(f, "Endpoint"),
            DescriptorType::Hid => write!(f, "Hid"),
            DescriptorType::Report => write!(f, "Report"),
        }
    }
}

impl EncodeByte for DescriptorType {
    fn encode(&self) -> Result<u8, &str> {
        match self {
            DescriptorType::Device => Ok(0x01),
            DescriptorType::Configuration => Ok(0x02),
            DescriptorType::String => Ok(0x03),
            DescriptorType::Interface => Ok(0x04),
            DescriptorType::Endpoint => Ok(0x05),
            DescriptorType::Hid => Ok(0x21),
            DescriptorType::Report => Ok(0x22),
        }   
    }
}

#[cfg(test)]
mod tests {
    use crate::{configuration::configuration_descriptor::CONFIGURATION_DESCRIPTOR_TYPE, device::device_descriptor::DEVICE_DESCRIPTOR_TYPE, endpoint::endpoint_descriptor::ENDPOINT_DESCRIPTOR_TYPE, interface::interface_descriptor::INTERFACE_DESCRIPTOR_TYPE};

    use super::*;

    const DESCRIPTOR_TYPE_DEVICE: u8 = 0x01;
    const DESCRIPTOR_TYPE_CONFIGURATION: u8 = 0x02;
    const DESCRIPTOR_TYPE_INTERFACE: u8 = 0x04;
    const DESCRIPTOR_TYPE_ENDPOINT: u8 = 0x05;

    #[test]
    fn test_encode() {
        let descriptor_type_device = DescriptorType::Device.encode().unwrap();
        assert_eq!(descriptor_type_device, DESCRIPTOR_TYPE_DEVICE);
        
        let descriptor_type_device = DEVICE_DESCRIPTOR_TYPE.encode().unwrap();
        assert_eq!(descriptor_type_device, DESCRIPTOR_TYPE_DEVICE);

        let descriptor_type_configuration = DescriptorType::Configuration.encode().unwrap();
        assert_eq!(descriptor_type_configuration, DESCRIPTOR_TYPE_CONFIGURATION);
        
        let descriptor_type_configuration = CONFIGURATION_DESCRIPTOR_TYPE.encode().unwrap();
        assert_eq!(descriptor_type_configuration, DESCRIPTOR_TYPE_CONFIGURATION);

        let descriptor_type_interface = DescriptorType::Interface.encode().unwrap();
        assert_eq!(descriptor_type_interface, DESCRIPTOR_TYPE_INTERFACE);
        
        let descriptor_type_interface = INTERFACE_DESCRIPTOR_TYPE.encode().unwrap();
        assert_eq!(descriptor_type_interface, DESCRIPTOR_TYPE_INTERFACE);

        let descriptor_type_endpoint = DescriptorType::Endpoint.encode().unwrap();
        assert_eq!(descriptor_type_endpoint, DESCRIPTOR_TYPE_ENDPOINT);
        
        let descriptor_type_endpoint = ENDPOINT_DESCRIPTOR_TYPE.encode().unwrap();
        assert_eq!(descriptor_type_endpoint, DESCRIPTOR_TYPE_ENDPOINT);
    }
}
