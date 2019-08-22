use super::memory;

#[allow(non_snake_case)]
pub struct CPU {
    // working registers
    B: u8, // 0
    C: u8, // 1
    D: u8, // 2
    E: u8, // 3
    H: u8, // 4
    L: u8, // 5
    A: u8, // 7 (accumulator)

    PC: u16,
    SP: u16,

    //TODO: add I/O
}