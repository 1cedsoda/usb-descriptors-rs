use alloc::vec::Vec;

use crate::{binary::EncodeByte, descriptor::Descriptor, descriptor_type::DescriptorType};

use super::interface_class::InterfaceClass;

pub const INTERFACE_DESCRIPTOR_LENGTH: u8 = 9;
pub const INTERFACE_DESCRIPTOR_TYPE: DescriptorType = DescriptorType::Interface;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDescriptor {
    /// Turns into `bInterfaceNumber`
    pub interface_number: u8,
    /// Turns into `bAlternateSetting`
    pub alternate_setting: u8,
    /// Turns into `bNumEndpoints`
    pub num_endpoints: u8,
    /// Turns into `bInterfaceClass`
    pub interface_class: InterfaceClass,
    /// Turns into `bInterfaceSubClass`
    pub interface_suclass: u8,
    /// Turns into `bInterfaceProtocol`
    pub interface_protocol: u8,
    /// Turns into `iInterface`
    pub interface: u8,
}

impl Descriptor for InterfaceDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(9); // bLength
        bytes.push(DescriptorType::Interface.encode()?); // bDescriptorType
        bytes.push(self.interface_number);
        bytes.push(self.alternate_setting);
        bytes.push(self.num_endpoints);
        bytes.push(self.interface_class.encode()?);
        bytes.push(self.interface_suclass);
        bytes.push(self.interface_protocol);
        bytes.push(self.interface);

        Ok(bytes)
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        DescriptorType::Interface
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        let interface_descriptor = InterfaceDescriptor {
            alternate_setting: 0,
            interface_class: InterfaceClass::HumanInterfaceDevice,
            interface_suclass: 0,
            interface_protocol: 0,
            interface_number: 0,
            num_endpoints: 1,
            interface: 1,
        };
        let interface_descriptor_encoded = vec![9, 4, 0, 0, 1, 3, 0, 0, 1];
        assert_eq!(
            interface_descriptor.encode().unwrap(),
            interface_descriptor_encoded
        );
    }
}
