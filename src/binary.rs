use crate::{
    error::Error,
    utils::{self, unlikely},
};
use std::slice;

pub struct Binary {
    pub(crate) header: Vec<u16>,
    pub(crate) map: Vec<u16>,
    pub(crate) object: Vec<u16>,
    pub(crate) str: Vec<u16>,
}

impl Binary {
    pub fn decode(data: &[u8]) -> Result<Binary, Error> {
        const END_OF_COMPRESS_CHUNK: [u8; 3] = [0, 0, 0];
        for (offset, val) in data.windows(END_OF_COMPRESS_CHUNK.len()).enumerate() {
            if val == END_OF_COMPRESS_CHUNK {
                let (compressed_ptr, str_ptr) = data.split_at(offset + END_OF_COMPRESS_CHUNK.len());
                let str = utils::u8_to_u16_and_destroy_u8(str_ptr.to_vec());

                let decompresed: Vec<u8> = Binary::decompress(compressed_ptr);

                const HEADER_SIZE: usize = 100;
                let decompressed_size: usize = decompresed.len();
                if unlikely(decompressed_size < HEADER_SIZE) {
                    return Err(Error::TooSmallLength {
                        length: decompressed_size,
                        least: HEADER_SIZE,
                    });
                }

                let (header_ptr, map_ptr) = decompresed.split_at(HEADER_SIZE);
                let header: Vec<u16> = utils::u8_to_u16_and_destroy_u8(header_ptr.to_vec());
                assert_eq!(header.len(), HEADER_SIZE / 2);

                let map_size: u16 = header[23];
                let map_area: u16 = map_size * map_size; // width * height
                let map_length: usize = usize::from(map_area * 2); // object_parts, background_parts

                let map: Vec<u16> = utils::u8_to_u16_and_destroy_u8(map_ptr[..map_length].to_vec());
                let property: Vec<u16> =
                    utils::u8_to_u16_and_destroy_u8(map_ptr[map_length..].to_vec());

                return Ok(Binary {
                    header,
                    map,
                    object: property,
                    str,
                });
            }
        }
        Err(Error::NoChunk)
    }

    fn decompress(ptr: &[u8]) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::with_capacity(ptr.len());
        let mut ptr_iter: slice::Windows<'_, u8> = ptr.windows(3);
        while let Some(b) = ptr_iter.next() {
            data.push(b[0]);
            if b[0] == b[1] {
                let loop_count: usize = b[2].into();
                data.append(&mut vec![b[0]; loop_count]);
                ptr_iter.nth(3);
            }
        }
        data
    }
}
