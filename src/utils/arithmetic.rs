const SIGN_BITMASK: u8 = 0b10000000;

// Checks if an unsigned number would be negative if it was signed. This is
// done by checking if the 7th bit is set.
#[inline(always)]
pub fn is_negative(byte: u8) -> bool {
    byte & SIGN_BITMASK == SIGN_BITMASK
}

#[inline(always)]
pub fn concat_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}
