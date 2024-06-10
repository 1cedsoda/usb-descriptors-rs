use alloc::vec::Vec;

use crate::{descriptor_type::DescriptorType, report::report_builder::ReportBuilder, version::Version};

use super::hid_descriptor::HidDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidBuilder {
    /// Turns into `bcdHID`
    pub bcd_hid: Version,
    /// Turns into `bCountryCode`
    pub country_code: u8,
    /// Turns into `bNumDescriptors`
    pub descriptors: Vec<ReportBuilder>
}

impl HidBuilder {
    pub fn build(&self, num_descriptors: u8, report_descriptor_length: u16) -> HidDescriptor {
        HidDescriptor {
            length: 9,
            descriptor_type: DescriptorType::Hid,
            bcd_hid: self.bcd_hid,
            country_code: self.country_code,
            num_descriptors,
            report_descriptor_type: DescriptorType::Report,
            report_descriptor_length,
        }
    }
}

// pub struct HidDescriptor {
//     /// Turns into `bLength`
//     pub length: u8,
//     /// Turns into `bDescriptorType`
//     pub descriptor_type: DescriptorType,
//     /// Turns into `bcdHID`
//     pub bcd_hid: Version,
//     /// Turns into `bCountryCode`
//     pub country_code: u8,
//     /// Turns into `bNumDescriptors`
//     pub num_descriptors: u8,
//     /// Turns into `bDescriptorType` for the report descriptor
//     pub report_descriptor_type: DescriptorType,
//     /// Turns into `wDescriptorLength`
//     pub report_descriptor_length: u16,
// }