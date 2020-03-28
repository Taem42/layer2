#![allow(dead_code)]
#![allow(unused_imports)]

mod consensus;
mod executor;
mod mempool;
mod merkle;
mod storage;
mod types;
mod utils;

use std::io::{Read, Write};
use std::time::Duration;

use futures::channel::mpsc;
use futures_timer::Delay;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::consensus::Consensus;
use crate::mempool::Mempool;
use crate::storage::Storage;
use crate::types::{Tx, Txs};

pub type MerkleRoot = Vec<u8>;

const GET_TXS_INTERVAL: u64 = 100;
static DATA_PATH: &str = "./data";
static TX_PATH: &str = "./tx.json";

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx_sender, tx_receiver) = mpsc::unbounded();
    let mempool = Mempool::new(tx_receiver).await;
    let storage = Storage::new(DATA_PATH);
    let mut consensus = Consensus::new(mempool, storage);

    tokio::spawn(async move {
        read_txs(tx_sender).await;
    });

    log::info!("Layer2 start running");
    consensus.run().await;
}

async fn read_txs(txs_sender: mpsc::UnboundedSender<Tx>) {
    loop {
        Delay::new(Duration::from_millis(GET_TXS_INTERVAL)).await;

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(TX_PATH)
            .expect("Could not open txs file");

        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Could not read txs file");
        file.write_all(&vec![]).expect("Could not write txs file");
        let txs: Txs = serde_json::from_str(&buf).expect("Could not deserialize txs from json");

        for tx in txs.to_inner().into_iter() {
            txs_sender.clone().unbounded_send(tx).unwrap();
        }
    }
}
