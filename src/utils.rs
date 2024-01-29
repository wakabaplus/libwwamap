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
