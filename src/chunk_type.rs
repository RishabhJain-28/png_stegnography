use std::{
    fmt::Display,
    str::{from_utf8, FromStr},
};

use anyhow::ensure;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }
    fn is_byte_valid(b: &u8) -> bool {
        b.is_ascii_uppercase() || b.is_ascii_lowercase()
    }
    fn is_chunk_valid(chunk: &[u8; 4]) -> bool {
        chunk.iter().all(Self::is_byte_valid)
    }
    fn is_valid(&self) -> bool {
        // Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.

        self.is_reserved_bit_valid() && Self::is_chunk_valid(&self.0)
    }
    fn is_critical(&self) -> bool {
        // for critical chunks 5th bit of first byte should be 0
        (self.0[0] & 32u8) == 0
    }
    fn is_public(&self) -> bool {
        // for public chunks 5th bit of second byte should be 0
        (self.0[1] & 32u8) == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        // reverved bit ie 5th bit of third byte should be 0
        (self.0[2] & 32u8) == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        // chunk is safe to copy if 5th bit of fourth byte is 1
        (self.0[3] & 32u8) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        ensure!(ChunkType::is_chunk_valid(&value), "Invalid chunk");
        let res = Self::new(value);
        Ok(res)
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.len() == 4, "invalid chunk length , should be 4 ");
        let bytes: [u8; 4] = s.as_bytes()[0..4].try_into()?;
        Self::try_from(bytes)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_utf8 = from_utf8(&self.0 as &[u8]);
        if str_utf8.is_err() {
            return Err(std::fmt::Error);
        }
        write!(f, "{}", str_utf8.unwrap())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());
        let chunk = ChunkType::from_str("Ru1t");

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
