use serde::{Deserialize, Serialize};

use crate::MerkleRoot;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub height:             u64,
    pub txs:                Vec<Tx>,
    pub median_state_roots: Vec<MerkleRoot>,
    pub latest_state_root:  MerkleRoot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tx {
    from:   u16,
    to:     u16,
    amount: u128,
}

impl Tx {
    pub fn new(from: u16, to: u16, amount: u128) -> Self {
        Tx { from, to, amount }
    }

    pub fn flatten(self) -> (usize, usize, u128) {
        (self.from as usize, self.to as usize, self.amount)
    }
}
