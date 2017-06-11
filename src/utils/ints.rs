pub fn is_larger(a : u32, b: u32) -> bool {
    a > b
}


const LINEAR_SCALE_STEP: u32 = 16843009; // = std::u32::MAX / (std::u8::MAX as u32)

pub fn linearscale(value: u32) -> u8 {
    (value / LINEAR_SCALE_STEP) as u8
}