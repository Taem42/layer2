use std::sync::Arc;

use futures::channel::mpsc;
use parking_lot::Mutex;

use crate::consensus::MempoolAdapter;
use crate::types::Tx;

pub struct Mempool {
    buffer: Arc<Mutex<Vec<Tx>>>,
}

impl MempoolAdapter for Mempool {
    fn get_txs(&self) -> Vec<Tx> {
        self.package()
    }
}

impl Mempool {
    pub async fn new(mut rx: mpsc::UnboundedReceiver<Tx>) -> Self {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);

        tokio::spawn(async move {
            loop {
                if let Ok(tx) = rx.try_next() {
                    if let Some(tx) = tx {
                        let mut buf = buffer_clone.lock();
                        buf.push(tx);
                    }
                }
            }
        });

        Mempool { buffer }
    }

    pub fn package(&self) -> Vec<Tx> {
        let mut buf = self.buffer.lock();
        let temp = buf.clone();
        buf.clear();
        temp
    }
}
