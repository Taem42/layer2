use futures::channel::mpsc;

use crate::merkle::MerkleTree;
use crate::types::Tx;
use crate::MerkleRoot;

pub struct Executor {
    merkle_tree: MerkleTree,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            merkle_tree: MerkleTree::new(),
        }
    }

    pub fn set_init_balance(&mut self, node: usize, balance: u128) {
        self.merkle_tree.set_balance(node, balance);
    }

    pub fn exec(&mut self, tx: Tx) -> MerkleRoot {
        let (from, to, amount) = tx.flatten();
        if from == to {
            self.set_init_balance(from, amount);
            self.merkle_tree.get_root()
        } else {
            self.merkle_tree.transfer(from, to, amount);
            self.merkle_tree.get_root()
        }
    }

    pub fn current_state_root(&mut self) -> MerkleRoot {
        self.merkle_tree.get_root()
    }
}
