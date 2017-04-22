const SIGN_BITMASK: u8 = 0b10000000;

// Checks if an unsigned number would be negative if it was signed. This is
// done by checking if the 7th bit is set.
#[inline(always)]
pub fn is_negative(arg: u8) -> bool {
    arg & SIGN_BITMASK == SIGN_BITMASK
}