use core::fmt::{Display, Formatter};

use alloc::{string::ToString, vec::Vec};

use super::{
    language_code::LanguageCode, string_content::StringContent, string_descriptor::StringDescriptor,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringBuidler {
    /// Turn into `bString`
    pub string: StringContent,
}

impl StringBuidler {
    pub fn text(string: &str) -> StringBuidler {
        StringBuidler {
            string: StringContent::Text(string.to_string()),
        }
    }
    pub fn languages(languages: Vec<LanguageCode>) -> StringBuidler {
        StringBuidler {
            string: StringContent::Languages(languages),
        }
    }
    pub fn build(&self, index: u8) -> StringDescriptor {
        StringDescriptor {
            index,
            length: self.string.len() + 2,
            string: self.string.clone(),
        }
    }
}

impl Display for StringBuidler {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.string {
            StringContent::Languages(languages) => {
                write!(f, "Languages: [")?;
                for language in languages {
                    write!(f, "{}, ", language)?;
                }
                write!(f, "]")
            }
            StringContent::Text(string) => write!(f, "Text: {}", string),
        }
    }
}
