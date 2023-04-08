use crate::chunk_type::ChunkType;
use crate::Result;
use anyhow::bail;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    chunk_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    chunk_crc: u32,
}

impl Chunk {
    pub const SIZE_WITHOUT_DATA: usize = 12;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let chunk_length: u32 = data.len().try_into().unwrap();
        let chunk_crc = Self::calculate_crc(&chunk_type.bytes().to_vec(), &data);

        let chunk: Chunk = Chunk {
            chunk_length: chunk_length,
            chunk_type: chunk_type,
            chunk_data: data,
            chunk_crc: chunk_crc,
        };
        chunk
    }

    pub fn length(&self) -> u32 {
        dbg!(self.chunk_length);
        self.chunk_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        let data: &[u8] = self.chunk_data.as_slice();
        data
    }

    pub fn crc(&self) -> u32 {
        self.chunk_crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = String::from_utf8(self.chunk_data.to_vec())?;
        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut self.chunk_length.to_be_bytes().to_vec());
        bytes.append(&mut self.chunk_type.bytes().to_vec());
        bytes.append(&mut self.chunk_data.clone());
        bytes.append(&mut self.chunk_crc.to_be_bytes().to_vec());
        bytes
    }

    pub fn calculate_crc(chunk_type: &Vec<u8>, chunk_data: &Vec<u8>) -> u32 {
        let mut crc_input = Vec::new();
        crc_input.append(&mut chunk_type.clone());
        crc_input.append(&mut chunk_data.clone());
        let crc: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let chunk_crc = crc.checksum(&crc_input);
        chunk_crc
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    /// construit à partir d'un tableau d'octets un chunk. Le tableau doit être complet :
    /// octets de longueur, octets du type, octets data et enfin octets CRC
    /// le CRC est recalculé pour s'assurer de la consistance du chunk
    fn try_from(array: &[u8]) -> std::result::Result<Self, Self::Error> {
        let chunk_length = u32::from_be_bytes([array[0], array[1], array[2], array[3]]);
        let chunk_type = ChunkType::try_from([array[4], array[5], array[6], array[7]])?;
        // let chunk_type = match r {
        //     Ok(ct) => ct,
        //     Err(_) => bail!("Invalid ChunkType"),
        // };
        let array_length = array.len();
        let data: &[u8] = &array[8..array_length - 4];
        let v_data = data.to_vec();
        let chunk_crc = u32::from_be_bytes([
            array[array_length - 4],
            array[array_length - 3],
            array[array_length - 2],
            array[array_length - 1],
        ]);
        let expected_crc = Chunk::calculate_crc(&chunk_type.bytes().to_vec(), &data.to_vec());
        if expected_crc != chunk_crc {
            bail!("CRC error");
        }

        let chunk = Chunk {
            chunk_length: chunk_length,
            chunk_type: chunk_type,
            chunk_data: v_data,
            chunk_crc: chunk_crc,
        };
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.chunk_length.to_string()
            + &" ".to_string()
            + &self.chunk_type.to_string()
            + &" ".to_string()
            + &self.data_as_string().unwrap()
            + &" ".to_string()
            + &self.chunk_crc.to_string();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
