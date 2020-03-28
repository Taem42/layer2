use bit_vec::BitVec;

use crate::MerkleRoot;

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

pub fn bits_extend_to_256(mut bits: Vec<bool>) -> Vec<bool> {
    assert!(bits.len() < 256);

    let len = 256 - bits.len();
    let mut temp = Vec::new();

    for _i in 0..len {
        temp.push(false);
    }

    bits.extend(temp.into_iter());
    bits
}

pub fn bits_to_bytes(bits: Vec<bool>) -> MerkleRoot {
    assert!(bits.len() % 8 == 0);
    let mut temp = BitVec::from_elem(bits.len(), false);

    for i in 0..bits.len() {
        temp.set(i, bits[i]);
    }

    temp.to_bytes()
}
