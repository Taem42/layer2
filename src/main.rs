#![allow(dead_code)]
#![allow(unused_imports)]

mod consensus;
mod executor;
mod mempool;
mod merkle;
mod storage;
mod types;
mod utils;

use std::io::{Read, Write, BufRead, BufReader};
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
static TX_PATH: &str = "./tx.list";

#[tokio::main]
async fn main() {
    env_logger::init();

    let txs_path = std::env::var("TXS_PATH").unwrap_or_else(|_| TX_PATH.to_string());

    let (tx_sender, tx_receiver) = mpsc::unbounded();
    let mempool = Mempool::new(tx_receiver).await;
    let storage = Storage::new(DATA_PATH);
    let mut consensus = Consensus::new(mempool, storage);

    tokio::spawn(async move {
        read_txs(tx_sender, txs_path).await;
    });

    log::info!("Layer2 start running");
    consensus.run().await;
}

// async fn tx_api(tx_sender: mpsc::UnboundedSender<Tx>) {
//     let stream = UnixStream::connect(TX_PATH)
//         .await
//         .expect("Could not connect to tx path");

//     let mut framed = FramedRead::new(stream, BytesCodec);
//     loop {
//         if let Some(bytes) = framed.try_next().await.unwrap() {
//             let txs: Txs =
//                 serde_json::from_slice(bytes).expect("Could not deserialize
// tx from json");             for tx in txs.to_inner().into_iter() {
//                 tx_sender.clone().unbounded_send(tx).unwrap();
//             }
//         }
//     }
// }

async fn read_txs(txs_sender: mpsc::UnboundedSender<Tx>, path: String) {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(path.clone())
        .expect("Could not open txs file");

    let mut reader = BufReader::new(file);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line.is_empty() || line.len() <= 2 {
            std::thread::sleep(std::time::Duration::from_secs(3));
            continue
        }
        let tx: Tx = serde_json::from_str(&line).expect("Could not deserialize txs from json");
        println!("{:?}", tx);
        txs_sender.clone().unbounded_send(tx).unwrap();
    }
}
