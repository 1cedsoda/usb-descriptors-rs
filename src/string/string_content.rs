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

#[cfg(test)]
mod tests {
    use string::ToString;

    use super::*;
    use crate::string::language_code::{DE_DE, EN_US};

    #[test]
    fn test_string_content_encode() {
        assert_eq!(
            StringContent::Text("Hello".to_string()).encode().unwrap(),
            vec![
                0x48, 0x00, // H
                0x65, 0x00, // e
                0x6C, 0x00, // l
                0x6C, 0x00, // l
                0x6F, 0x00, // o
            ]
        );
        assert_eq!(
            StringContent::Languages(vec![EN_US, DE_DE])
                .encode()
                .unwrap(),
            vec![
                0x09, 0x04, // EN_US
                0x07, 0x04, // DE_DE
            ]
        );
    }
}
