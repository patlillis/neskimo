const SIGN_BITMASK: u8 = 0b10000000;

// Checks if an unsigned number would be negative if it was signed. This is
// done by checking if the 7th bit is set.
pub fn is_negative(byte: u8) -> bool {
    byte & SIGN_BITMASK == SIGN_BITMASK
}

// Combne low and high bytes into a u16.
pub fn concat_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

// Adds a relative displacement to an address. This is useful for operations
// using relative addressing that allow branching forwards or backwards.
pub fn add_relative(base_address: u16, displacement: i8) -> u16 {
    if displacement < 0 {
        base_address.wrapping_sub(-(displacement) as u16)
    } else {
        base_address.wrapping_add(displacement as u16)
    }
}