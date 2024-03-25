use crate::{
    error::Error,
    ext::{U8Ext, VecExt},
    utils::unlikely,
};
use core::slice;

pub struct Binary {
    pub(crate) header: Vec<u16>,
    pub(crate) map_tiled: Vec<u16>,
    pub(crate) object_tiled: Vec<u16>,
    pub(crate) map_entity: Vec<u16>,
    pub(crate) object_entity: Vec<u16>,
    pub(crate) str: Vec<u16>,
}

#[derive(PartialEq)]
pub enum BinaryOption {
    IgnoreChecksum,
}

impl Binary {
    pub fn decode(data: &[u8], option: Option<BinaryOption>) -> Result<Binary, Error> {
        const END_OF_COMPRESS_MAGIC: [u8; 3] = [0, 0, 0];
        for (offset, val) in data.windows(END_OF_COMPRESS_MAGIC.len()).enumerate() {
            if val == END_OF_COMPRESS_MAGIC {
                let (compressed_ptr, str_ptr) = data.split_at(offset + END_OF_COMPRESS_MAGIC.len());
                let decompresed = Binary::decompress(compressed_ptr);
                if option != Some(BinaryOption::IgnoreChecksum) {
                    Binary::validate(&decompresed)?;
                }

                const HEADER_SIZE: usize = 90;
                let (header_ptr, map_ptr) = decompresed.split_at(HEADER_SIZE);
                let header = header_ptr.to_vec().into_u16();

                let map_size = header[23];
                let map_area = usize::from(map_size * map_size * 2); // width * height * WORD
                let (map_array, object_array) = map_ptr.split_at(map_area);
                let (object_array, map_entity) = object_array.split_at(map_area);

                const CHUNK_SIZE: usize = 120;
                let map_entity_size = usize::from(header[17]) * CHUNK_SIZE;
                let (map_entity, object_entity) = map_entity.split_at(map_entity_size);
                let object_entity_size = usize::from(header[18]) * CHUNK_SIZE;
                let (object_entity, _) = object_entity.split_at(object_entity_size);

                return Ok(Binary {
                    header,
                    map_tiled: map_array.to_vec().into_u16(),
                    object_tiled: object_array.to_vec().into_u16(),
                    map_entity: map_entity.to_vec().into_u16(),
                    object_entity: object_entity.to_vec().into_u16(),
                    str: str_ptr.to_vec().into_u16(),
                });
            }
        }
        Err(Error::NoChunk)
    }

    fn decompress(ptr: &[u8]) -> Vec<u8> {
        // Decompressed size will be larger than compressed size.
        // Therefore, we allocate compressed size in advance.
        let mut data: Vec<u8> = Vec::with_capacity(ptr.len());
        let mut ptr_iter: slice::Windows<'_, u8> = ptr.windows(3);

        // Don't consider checksum (first 2 bytes) as compressed data.
        data.push(ptr_iter.next().unwrap()[0]);
        data.push(ptr_iter.next().unwrap()[0]);
        while let Some(b) = ptr_iter.next() {
            data.push(b[0]);
            if b[0] == b[1] {
                let loop_count: usize = b[2].into();
                data.append(&mut vec![b[0]; loop_count]);
                ptr_iter.nth(1);
            }
        }
        data
    }

    fn validate(bin: &Vec<u8>) -> Result<(), Error> {
        let correct_checksum: u16 = [bin[0], bin[1]].into_u16();
        let mut checksum: i32 = 0;

        bin.iter().enumerate().skip(2).for_each(|(n, byte)| {
            // Even if n > i32::MAX, since n % 8, it's used only for the lower 4 bits.
            checksum += i32::from(*byte as i8) * (n as i32 % 8 + 1);
        });
        let checksum: u16 = checksum as u16;
        if unlikely(correct_checksum != checksum) {
            Err(Error::InvalidChecksum {
                except: correct_checksum,
                actual: checksum,
            })
        } else {
            Ok(())
        }
    }
}
