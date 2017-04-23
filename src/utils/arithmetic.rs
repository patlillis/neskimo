const SIGN_BITMASK: u8 = 0b10000000;

// Checks if an unsigned number would be negative if it was signed. This is
// done by checking if the 7th bit is set.
#[inline(always)]
pub fn is_negative(byte: u8) -> bool {
    byte & SIGN_BITMASK == SIGN_BITMASK
}

#[inline(always)]
pub fn concat_bytes(byte1: u8, byte2: u8) -> u16 {
    ((byte1 as u16) << 8) | (byte2 as u16)
}
