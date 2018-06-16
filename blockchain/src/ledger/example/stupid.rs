//! # Stupid Ledger
//!
//! Stupid Ledger is a distributed ledger implemented with vanilla blockchain.

use objecthash;
use objecthash::{ObjectHash, ObjectHasher};
use ledger::*;

#[derive(Debug)]
pub struct StupidLedger {
    chain: Vec<StupidBlock>,
    block_txs: Vec<StupidTransaction>
}

impl StupidLedger {
    pub fn new(chain: Vec<StupidBlock>) -> StupidLedger {
        StupidLedger { chain, block_txs: Vec::new() }
    }
}

impl DistributedLedger<StupidBlock, StupidTransaction> for StupidLedger {
    type LedgerRepr = Vec<StupidBlock>;
    type Proof = usize;

    fn new_block(&mut self, timestamp: Duration, proof: Self::Proof) -> Result<&Self::LedgerRepr, BlockError> {
        let block = StupidBlock::new(
            self.chain.len() + 1,
            Timestamp::new(timestamp),
            self.block_txs.to_vec(),
            proof,
            if self.chain.len() == 0 {
                vec![0; 32]
            } else {
                Self::hash(self.last_block().unwrap())
            }
        );
        self.block_txs.clear();
        self.chain.push(block);
        Ok(&self.chain)
    }

    fn add_transaction(&mut self, tx: StupidTransaction) -> Result<usize, TransactionError> {
        self.block_txs.push(tx);
        Ok(self.chain.len() + 1)
    }

    fn last_block(&self) -> Option<&StupidBlock> {
        self.chain.last()
    }

    fn hash(obj: &StupidBlock) -> Vec<u8> {
        objecthash::digest(obj).as_ref().to_vec()
    }
}

/// Defines a stupid block of transactions in the blockchain.
#[derive(Debug, PartialEq)]
pub struct StupidBlock {
    index: usize,
    timestamp: Timestamp,
    transactions: Vec<StupidTransaction>,
    proof: usize,
    previous_hash: Vec<u8>
}

impl StupidBlock {
    pub fn new(index: usize, timestamp: Timestamp, transactions: Vec<StupidTransaction>,
        proof: usize, previous_hash: Vec<u8>) -> StupidBlock
    {
        StupidBlock { index, timestamp, transactions, proof, previous_hash }
    }
}

/// Makes StupidBlock hashable.
impl ObjectHash for StupidBlock {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "index" => &self.index,
            "timestamp" => &self.timestamp,
            "transactions" => &self.transactions,
            "proof" => &self.proof,
            "previous_hash" => &self.previous_hash
        )
    }
}

impl Block for StupidBlock {
    fn is_valid(&self) -> bool {
        true
    }
}

/// Defines a stupid transaction in the blockchain.
#[derive(Debug, PartialEq, Clone)]
pub struct StupidTransaction {
    sender: String,
    recipient: String,
    amount: usize
}

impl StupidTransaction {
    pub fn new(sender: String, recipient: String, amount: usize) -> StupidTransaction {
        StupidTransaction { sender, recipient, amount }
    }
}

/// Makes StupidTransaction hashable.
impl ObjectHash for StupidTransaction {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "sender" => &self.sender,
            "recipient" => &self.recipient,
            "amount" => &self.amount
        )
    }
}

impl Transaction for StupidTransaction {
    fn is_valid(&self) -> bool {
        true
    }
}

/// Newtype for std::time::Duration.
#[derive(Debug, PartialEq, Clone)]
pub struct Timestamp(Duration);

impl Timestamp {
    pub fn new(value: Duration) -> Timestamp {
        Timestamp(value)
    }
}

/// Makes Timestamp hashable.
impl ObjectHash for Timestamp {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "value" => &(self.0.as_secs()*(1e9 as u64) + self.0.subsec_nanos() as u64)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn add_transaction() {
        let tx = create_transaction();
        let mut stupid_chain = StupidLedger::new(Vec::new());
        assert_eq!(stupid_chain.add_transaction(tx.clone()).unwrap(), 1);
    }

    #[test]
    fn new_block() {
        let tx = create_transaction();
        let mut stupid_chain = StupidLedger::new(Vec::new());
        stupid_chain.add_transaction(tx.clone()).expect("bad transaction");

        let duration = Duration::new(12345, 0);
        let timestamp = Timestamp::new(duration.clone());
        assert_eq!(stupid_chain.new_block(duration, 1000).unwrap()[0],
            StupidBlock::new(1, timestamp, vec![tx.clone()], 1000, vec![0; 32]));
    }

    fn create_transaction() -> StupidTransaction {
        let sender = String::from("0x0001");
        let recipient = String::from("0x0002");
        let amount = 10000;
        StupidTransaction::new(sender, recipient, amount)
    }
}
