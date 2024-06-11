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

#[cfg(test)]
mod tests {
    use crate::string::language_code::{DE_DE, EN_US};

    use super::*;

    #[test]
    fn test_string_builder_text() {
        let string = StringBuidler::text("Hello, World!");
        assert_eq!(string.to_string(), "Text: Hello, World!");
        assert_eq!(string.build(1).string, StringContent::Text("Hello, World!".to_string()));
    }

    #[test]
    fn test_string_builder_languages() {
        let languages = StringContent::Languages(vec![EN_US, DE_DE]);
        let string = StringBuidler { string: languages };
        assert_eq!(string.to_string(), "Languages: [0x0409, 0x0407, ]");
    }

    #[test]
    fn test_string_builder_build() {
        let string = StringBuidler::text("Hello, World!");
        let descriptor = string.build(1);
        assert_eq!(descriptor.index, 1);
        assert_eq!(descriptor.string, StringContent::Text("Hello, World!".to_string()));
    }
}
