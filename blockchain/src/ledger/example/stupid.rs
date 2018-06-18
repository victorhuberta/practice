//! # Stupid Ledger
//!
//! Stupid Ledger is a distributed ledger implemented with vanilla blockchain.

use objecthash;
use objecthash::{ObjectHash, ObjectHasher};
use ledger::*;
use ledger::util::Hex;

#[derive(Debug, Serialize, Deserialize)]
pub struct StupidLedger {
    pub chain: Vec<StupidBlock>,
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

    fn new_block(&mut self, timestamp: Timestamp, proof: Self::Proof) -> Result<&Self::LedgerRepr, BlockError> {
        let block = StupidBlock::new(
            self.chain.len() + 1,
            timestamp,
            self.block_txs.to_vec(),
            proof,
            if let Some(last_block) = self.last_block() {
                Self::hash(last_block)
            } else {
                vec![0; 32]
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

    fn find_proof(&self, last_proof: Self::Proof) -> Self::Proof {
        let last_block_hash = if let Some(last_block) = self.last_block() {
            Self::hash(last_block)
        } else {
            vec![0; 32]
        };
        let mut proof = 0;
        while ! Self::is_valid_proof(last_block_hash.to_vec(), last_proof, proof) {
            proof += 1;
        }
        proof
    }

    fn is_valid_proof(last_block_hash: Vec<u8>, last_proof: Self::Proof, proof: Self::Proof) -> bool {
        let last_block_hash = Hex::from_bytes(&last_block_hash[..]);
        let s = format!("{}{}{}", last_block_hash, last_proof, proof);
        objecthash::digest(&s).as_ref()[..4] == [0, 0, 0, 0]
    }
}

/// Defines a stupid block of transactions in the blockchain.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StupidBlock {
    index: usize,
    timestamp: Timestamp,
    transactions: Vec<StupidTransaction>,
    pub proof: usize,
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

        let timestamp = Timestamp::new(Duration::new(12345, 0));
        assert_eq!(stupid_chain.new_block(timestamp.clone(), 1000).unwrap()[0],
            StupidBlock::new(1, timestamp, vec![tx.clone()], 1000, vec![0; 32]));
    }

    fn create_transaction() -> StupidTransaction {
        let sender = String::from("0x0001");
        let recipient = String::from("0x0002");
        let amount = 10000;
        StupidTransaction::new(sender, recipient, amount)
    }
}
