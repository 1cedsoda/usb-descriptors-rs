use alloc::vec::Vec;

use crate::descriptor_type::DescriptorType;

pub trait Descriptor {
    fn encode(&self) -> Result<Vec<u8>, &str>;
    fn get_descriptor_type(&self) -> DescriptorType;
}
