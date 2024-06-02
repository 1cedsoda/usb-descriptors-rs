use alloc::vec::Vec;

use crate::{binary::EncodeByte, descriptor::Descriptor, descriptor_type::DescriptorType};

use super::interface_device_class::InterfaceDeviceClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDescriptor {
    pub b_length: u8,
    pub b_descriptor_type: DescriptorType,
    pub b_interface_number: u8,
    pub b_alternate_setting: u8,
    pub b_num_endpoints: u8,
    pub b_interface_class: InterfaceDeviceClass,
    pub b_interface_sub_class: u8,
    pub b_interface_protocol: u8,
    pub i_interface: u8,
}

impl Descriptor for InterfaceDescriptor {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        bytes.push(9);
        bytes.push(DescriptorType::Interface.encode()?);
        bytes.push(self.b_interface_number);
        bytes.push(self.b_alternate_setting);
        bytes.push(self.b_num_endpoints);
        bytes.push(self.b_interface_class.encode()?);
        bytes.push(self.b_interface_sub_class);
        bytes.push(self.b_interface_protocol);
        bytes.push(self.i_interface);
        
        if bytes.len() != self.b_length as usize {
            return Err("b_length does not match the actual length");
        }

        Ok(bytes)
    }

    fn get_w_value(&self) -> u16 {
        (self.b_descriptor_type.encode().unwrap() as u16) << 8 | self.b_interface_number as u16
    }
    
    fn get_descriptor_type(&self) -> DescriptorType {
        self.b_descriptor_type
    }
}

#[cfg(test)]
pub mod tests {

    use crate::interface::interface::Interface;

    use super::*;

    #[test]
    fn test_encode() {
        let interface_descriptor = Interface {
            b_alternate_setting: 0,
            b_interface_class: InterfaceDeviceClass::HumanInterfaceDevice,
            b_interface_sub_class: 0,
            b_interface_protocol: 0,
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
