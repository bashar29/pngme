use std::fmt::Display;

use anyhow::bail;



#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    type_str : String,
}

impl ChunkType {
    
    /// Convert internal representation of ChunkType in an array of u8
    fn bytes(&self) -> [u8; 4] {
        let mut bytes:[u8; 4] = [0,0,0,0];
        let mut cpt = 0; 
        for b in self.type_str.as_bytes() {
            bytes[cpt] = *b;
            cpt += 1;
        }
        bytes
    }

    /// Valid if reserved bit is valid and all bytes are represented
    /// by A-Z or a-z characters
    fn is_valid(&self) -> bool {
        if self.is_reserved_bit_valid() == true {
            for byte in self.bytes() {
                if byte.is_ascii_lowercase() == false && byte.is_ascii_uppercase() == false {
                    return false;
                }
            }
        }
        else {
            return false;
        }
        true
    }

    /// Spec : 
    /// Ancillary bit: bit 5 of first byte
    /// 0 (uppercase) = critical, 1 (lowercase) = ancillary.
    /// # Examples
    /// ChunkType::from_str("RuSt").unwrap();
    //  assert_eq!(expected, actual); 
    fn is_critical(&self) -> bool {
        let byte_to_check = self.bytes()[0];
        if byte_to_check & 0b00100000 == 32 {
            return false;
        }
        true
    }

    /// Spec : Private bit: bit 5 of second byte
    /// 0 (uppercase) = public, 1 (lowercase) = private.
    fn is_public(&self) -> bool {
        let byte_to_check = self.bytes()[1];
        if byte_to_check & 0b00100000 == 32 {
            return false;
        }
        true
    }

    /// Reserved bit: bit 5 of third byte
    /// Must be 0 (uppercase) in files conforming to this version of PNG.
    fn is_reserved_bit_valid(&self) -> bool {
        let byte_to_check = self.bytes()[2];
        if byte_to_check & 0b00100000 == 32 {
            return false;
        }
        true
    }

    ///Safe-to-copy bit: bit 5 of fourth byte
    /// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
    fn is_safe_to_copy(&self) -> bool {
        let byte_to_check = self.bytes()[3];
        if byte_to_check & 0b00100000 == 0 {
            return false;
        }
        true
    }


}

impl TryFrom<[u8; 4]> for ChunkType {

    type Error = crate::Error;

    fn try_from(array: [u8; 4]) -> Result<Self, Self::Error> {

        for b in array {
            if b.is_ascii() == false {
                bail!("no ASCII input");
            }
        }
        let v = array.to_vec();
        let s: String = String::from_utf8(v).unwrap();
        let ct: ChunkType = ChunkType{type_str: s};
        Ok(ct)
    }
}

impl std::str::FromStr for ChunkType {
    
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for b in s.as_bytes() {
            if b.is_ascii_lowercase() == false &&  b.is_ascii_uppercase() == false {
                bail!("no ASCII input");
            }
        }
        let ct: ChunkType = ChunkType{type_str: s.to_string()};
        Ok(ct)     
    }
}

impl Display for ChunkType {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_str)
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

