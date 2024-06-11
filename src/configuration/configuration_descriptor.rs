use alloc::vec::Vec;

use crate::{binary::EncodeByte, descriptor::Descriptor, descriptor_type::DescriptorType};

use super::{configuration_attributes::ConfigurationAttributes, milliamperes::Milliamperes};

pub const CONFIGURATION_DESCRIPTOR_LENGTH: u8 = 9;
pub const CONFIGURATION_DESCRIPTOR_TYPE: DescriptorType = DescriptorType::Configuration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationDescriptor {
    /// Turns into `wTotalLength`
    pub total_length: u16,
    /// Turns into `bNumInterfaces`
    pub num_interfaces: u8,
    /// Turns into `bConfigurationValue`
    pub configuration_value: u8,
    /// Turns into `iConfiguration`
    pub configuration: u8,
    /// Turns into `bmAttributes`
    pub attributes: ConfigurationAttributes,
    /// Turns into `bMaxPower`
    pub max_power: Milliamperes,
}

impl Descriptor for ConfigurationDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(CONFIGURATION_DESCRIPTOR_LENGTH);
        bytes.push(CONFIGURATION_DESCRIPTOR_TYPE.encode()?);
        bytes.extend_from_slice(&self.total_length.to_le_bytes());
        bytes.push(self.num_interfaces);
        bytes.push(self.configuration_value);
        bytes.push(self.configuration);
        bytes.push(self.attributes.encode()?);
        bytes.push(self.max_power.encode()?);

        if bytes.len() != CONFIGURATION_DESCRIPTOR_LENGTH as usize {
            return Err("configuration bLength not match the actual length");
        }

        Ok(bytes)
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        CONFIGURATION_DESCRIPTOR_TYPE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let descriptor = ConfigurationDescriptor {
            total_length: 34,
            num_interfaces: 1,
            configuration_value: 2,
            configuration: 3,
            attributes: ConfigurationAttributes {
                self_powered: true,
                remote_wakeup: false,
            },
            max_power: Milliamperes(50),
        };

        assert_eq!(
            descriptor.encode().unwrap(),
            vec![
                9,           // bLength
                2,           // bDescriptorType
                34,          // wTotalLength
                0,           //
                1,           // bNumInterfaces
                2,           // bConfigurationValue
                3,           // iConfiguration
                0b1100_0000, // bmAttributes
                25,          // bMaxPower
            ]
        );
    }
}
