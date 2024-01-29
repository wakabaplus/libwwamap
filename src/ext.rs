pub trait VecExt<T> {
    fn into_u16(self) -> Vec<u16>;
}

impl VecExt<u8> for Vec<u8> {
    #[inline]
    fn into_u16(self) -> Vec<u16> {
        self.chunks_exact(2)
            .map(|a| u16::from_le_bytes([a[0], a[1]]))
            .collect()
    }
}

pub trait U8Ext {
    fn into_u16(self) -> u16;
}

impl U8Ext for [u8; 2] {
    #[inline]
    fn into_u16(self) -> u16 {
        u16::from(self[0]) + u16::from(self[1]) * 0x100
    }
}