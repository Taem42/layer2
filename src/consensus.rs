use std::time::Duration;

use futures_timer::Delay;
use log::info;
use serde_json::to_string_pretty;

use crate::executor::Executor;
use crate::types::{Block, Tx};
use crate::MerkleRoot;

const BLOCK_INTERVAL: u64 = 3000;
const INIT_HEIGHT: u64 = 0;

pub trait MempoolAdapter {
    fn get_txs(&self) -> Vec<Tx>;
}

pub trait StorageAdapter {
    fn write(&self, height: u64, data: String);
}

pub struct Consensus<M, S> {
    height:   u64,
    mempool:  M,
    storage:  S,
    executor: Executor,
}

impl<M, S> Consensus<M, S>
where
    M: MempoolAdapter,
    S: StorageAdapter,
{
    pub fn new(mempool: M, storage: S) -> Self {
        Consensus {
            height: INIT_HEIGHT,
            executor: Executor::new(),
            mempool,
            storage,
        }
    }

    pub async fn run(&mut self) {
        self.height += 1;
        loop {
            Delay::new(Duration::from_millis(BLOCK_INTERVAL)).await;

            let txs = self.mempool.get_txs();
            let tx_count = txs.len();
            let state_roots = if txs.is_empty() {
                vec![self.executor.current_state_root()]
            } else {
                txs.iter()
                    .map(|tx| self.executor.exec(tx.clone()))
                    .collect::<Vec<_>>()
            };

            let block = self.generate_block(txs, state_roots);
            self.storage
                .write(self.height, to_string_pretty(&block).unwrap());

            info!(
                "layer 2 height {}, transaction count {}",
                self.height, tx_count
            );
            self.height += 1;
        }
    }

    fn generate_block(&self, txs: Vec<Tx>, state_roots: Vec<MerkleRoot>) -> Block {
        assert!(!state_roots.is_empty());
        Block {
            height: self.height,
            latest_state_root: state_roots.last().unwrap().clone(),
            median_state_roots: state_roots,
            txs,
        }
    }
}
