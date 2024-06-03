use alloc::{string::ToString, vec::Vec};

use crate::descriptor_type::DescriptorType;

use super::{
    language_code::LanguageCode, string_content::StringContent, string_descriptor::StringDescriptor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringIntrinsics {
    /// Turn into `bString`
    pub string: StringContent,
}

impl StringIntrinsics {
    pub fn text(string: &str) -> StringIntrinsics {
        StringIntrinsics {
            string: StringContent::Text(string.to_string()),
        }
    }
    pub fn languages(languages: Vec<LanguageCode>) -> StringIntrinsics {
        StringIntrinsics {
            string: StringContent::Languages(languages),
        }
    }
    pub fn build(&self, index: u8) -> StringDescriptor {
        StringDescriptor {
            index,
            length: self.string.len(),
            descriptor_type: DescriptorType::String,
            string: self.string.clone(),
        }
    }
}
