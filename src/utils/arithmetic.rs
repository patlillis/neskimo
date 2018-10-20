const SIGN_BITMASK: u8 = 0b1000_0000;

// Checks if an unsigned number would be negative if it was signed. This is
// done by checking if the 7th bit is set.
pub fn is_negative(byte: u8) -> bool {
    byte & SIGN_BITMASK == SIGN_BITMASK
}

// Combne low and high bytes into a u16.
pub fn concat_bytes(high: u8, low: u8) -> u16 {
    (u16::from(high) << 8) | u16::from(low)
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
