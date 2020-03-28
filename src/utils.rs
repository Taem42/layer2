pub fn convert_u128_to_bits(num: u128) -> Vec<bool> {
    num.to_le_bytes()
        .iter()
        .map(|byte| (0..8).map(move |i| (byte >> i) & 1u8 == 1u8).rev())
        .flatten()
        .collect()
}

pub fn convert_u16_to_bits(num: u16) -> Vec<bool> {
    num.to_le_bytes()
        .iter()
        .map(|byte| (0..8).map(move |i| (byte >> i) & 1u8 == 1u8).rev())
        .flatten()
        .collect()
}
