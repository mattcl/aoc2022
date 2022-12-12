#[inline]
pub fn char_to_mask(ch: char) -> u64 {
    let v = if ch.is_lowercase() {
        ((ch as u8) - ('a' as u8)) as usize
    } else {
        ((ch as u8) - ('A' as u8) + 26) as usize
    };
    mask(v)
}

#[inline]
pub fn mask(shift: usize) -> u64 {
    1 << shift
}

#[inline]
pub fn char_to_num(ch: char) -> u8 {
    if ch.is_lowercase() {
        (ch as u8) - ('a' as u8)
    } else {
        (ch as u8) - ('A' as u8) + 26
    }
}
