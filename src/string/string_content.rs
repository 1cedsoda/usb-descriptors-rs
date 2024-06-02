use alloc::{
    string::{self},
    vec::Vec,
};

use crate::binary::EncodeBytes;

use super::language_code::LanguageCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringContent {
    Languages(Vec<LanguageCode>),
    Text(string::String),
}

impl StringContent {
    pub fn len(&self) -> u8 {
        match self {
            StringContent::Languages(languages) => languages.len() as u8 * 2,
            StringContent::Text(string) => string.len() as u8 * 2,
        }
    }
}

impl EncodeBytes for StringContent {
    fn encode(&self) -> Result<Vec<u8>, &str> {
        let mut bytes = Vec::<u8>::new();
        match self {
            StringContent::Languages(languages) => {
                for language in languages {
                    bytes.append(language.encode()?.as_mut());
                }
            }
            StringContent::Text(string) => {
                for c in string.encode_utf16() {
                    bytes.append(&mut c.to_le_bytes().to_vec());
                }
            }
        }
        Ok(bytes)
    }
}
