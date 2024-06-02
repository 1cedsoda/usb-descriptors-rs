use alloc::{string::ToString, vec::Vec};

use crate::descriptor_type::DescriptorType;

use super::{
    language_code::LanguageCode, string_content::StringContent, string_descriptor::StringDescriptor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    pub b_string: StringContent,
}

impl String {
    pub fn text(string: &str) -> String {
        String {
            b_string: StringContent::Text(string.to_string()),
        }
    }
    pub fn languages(languages: Vec<LanguageCode>) -> String {
        String {
            b_string: StringContent::Languages(languages),
        }
    }
    pub fn build(&self, index: u8) -> StringDescriptor {
        StringDescriptor {
            index,
            b_length: self.b_string.len(),
            b_descriptor_type: DescriptorType::String,
            b_string: self.b_string.clone(),
        }
    }
}
