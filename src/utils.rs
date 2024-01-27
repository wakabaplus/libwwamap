use crate::tiled::Map;

#[inline]
pub fn u8vec_to_u16vec_and_destroy_u8vec(v: Vec<u8>) -> Vec<u16> {
    v.chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect()
}

#[inline]
pub fn u8arr_to_u16(v: [u8; 2]) -> u16 {
    u16::from(v[0]) + u16::from(v[1]) * 0x100
}

#[inline]
pub fn u16_to_first_u8_le(v: u16) -> u8 {
    v.to_le_bytes()[0]
}

#[inline]
pub fn xy_to_id(coord: Map) -> u16 {
    10 * coord.x + coord.y
}

// https://users.rust-lang.org/t/compiler-hint-for-unlikely-likely-for-if-branches/62102/4
#[inline(always)]
#[cold]
pub const fn cold() {}

#[inline(always)]
pub const fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}
