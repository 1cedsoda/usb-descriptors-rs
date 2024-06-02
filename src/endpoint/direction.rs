use crate::binary::EncodeByte;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Out = 0x00,
    In = 0x01,
}

impl EncodeByte for Direction {
    fn encode(&self) -> Result<u8, &str> {
        Ok(*self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIRECTION_OUT: u8 = 0x00;
    const DIRECTION_IN: u8 = 0x01;

    #[test]
    fn test_encode() {
        assert_eq!(Direction::Out.encode().unwrap(), DIRECTION_OUT);
        assert_eq!(Direction::In.encode().unwrap(), DIRECTION_IN);
    }
}
