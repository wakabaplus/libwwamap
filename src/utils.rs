use crate::index::Map;

#[inline]
pub fn u8_to_u16_and_destroy_u8(v: Vec<u8>) -> Vec<u16> {
    v.chunks_exact(2)
        .map(|a| u16::from_le_bytes([a[0], a[1]]))
        .collect()
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
