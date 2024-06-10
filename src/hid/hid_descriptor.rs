use crate::{
    binary::EncodeBytes, descriptor::Descriptor, descriptor_type::DescriptorType, version::Version,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidDescriptor {
    /// Turns into `bLength`
    pub length: u8,
    /// Turns into `bDescriptorType`
    pub descriptor_type: DescriptorType,
    /// Turns into `bcdHID`
    pub bcd_hid: Version,
    /// Turns into `bCountryCode`
    pub country_code: u8,
    /// Turns into `bNumDescriptors`
    pub num_descriptors: u8,
    /// Turns into `bDescriptorType` for the report descriptor
    pub report_descriptor_type: DescriptorType,
    /// Turns into `wDescriptorLength`
    pub report_descriptor_length: u16,
}

impl Descriptor for HidDescriptor {
    fn encode(&self) -> Result<alloc::vec::Vec<u8>, &str> {
        let mut encoded = alloc::vec::Vec::new();
        encoded.push(self.length);
        encoded.push(self.descriptor_type as u8);
        encoded.extend_from_slice(&self.bcd_hid.encode()?);
        encoded.push(self.country_code);
        encoded.push(self.num_descriptors);
        encoded.push(self.report_descriptor_type as u8);
        encoded.extend_from_slice(&self.report_descriptor_length.to_le_bytes());
        Ok(encoded)
    }

    fn get_descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
}
