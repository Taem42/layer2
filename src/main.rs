#![allow(dead_code)]
#![allow(unused_imports)]

mod consensus;
mod executor;
mod mempool;
mod merkle;
mod storage;
mod types;
mod utils;

use futures::channel::mpsc;
use sha2::{Digest, Sha256};

use crate::consensus::Consensus;
use crate::mempool::Mempool;
use crate::storage::Storage;

pub type MerkleRoot = Vec<u8>;

static DATA_PATH: &str = "./data";

// fn default_genesis_root() -> MerkleRoot {
//     let mut hasher = Sha256::new();
//     hasher.input(b"team 42 win");
//     let result = hasher.result();
//     result[..].to_vec()
// }

#[tokio::main]
async fn main() {
    let (tx_sender, tx_receiver) = mpsc::unbounded();
    let mempool = Mempool::new(tx_receiver).await;
    let storage = Storage::new(DATA_PATH);
    let mut consensus = Consensus::new(mempool, storage);

    consensus.run().await;
}
