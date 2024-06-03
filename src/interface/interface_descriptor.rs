use alloc::vec::Vec;

use crate::{binary::EncodeByte, descriptor::Descriptor, descriptor_type::DescriptorType};

use super::interface_device_class::InterfaceDeviceClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDescriptor {
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// Turns into `bInterfaceNumber`
    pub interface_number: u8,
    /// Turns into `bAlternateSetting`
    pub alternate_setting: u8,
    /// Turns into `bNumEndpoints`
    pub num_endpoints: u8,
    /// Turns into `bInterfaceClass`
    pub interface_class: InterfaceDeviceClass,
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
        bytes.push(9);
        bytes.push(DescriptorType::Interface.encode()?);
        bytes.push(self.interface_number);
        bytes.push(self.alternate_setting);
        bytes.push(self.num_endpoints);
        bytes.push(self.interface_class.encode()?);
        bytes.push(self.interface_suclass);
        bytes.push(self.interface_protocol);
        bytes.push(self.interface);

        if bytes.len() != self.length as usize {
            return Err("length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.descriptor_type.encode().unwrap() as u16) << 8 | self.interface_number as u16
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
}

#[cfg(test)]
pub mod tests {

    use crate::interface::interface_intrinics::InterfaceIntrinics;

    use super::*;

    #[test]
    fn test_encode() {
        let interface_descriptor = InterfaceIntrinics {
            alternate_setting: 0,
            interface_class: InterfaceDeviceClass::HumanInterfaceDevice,
            interface_suclass: 0,
            interface_protocol: 0,
        }
        .build(0, 0, 1)
        .unwrap();
        let interface_descriptor_encoded = vec![9, 4, 0, 0, 1, 3, 0, 0, 0];
        assert_eq!(
            interface_descriptor.encode().unwrap(),
            interface_descriptor_encoded
        );
    }
}
