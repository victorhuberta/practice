#[macro_use]
extern crate objecthash;

use std::error::Error;
use std::fmt;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use objecthash::{ObjectHash, ObjectHasher};

type LedgerRepr = Vec<Block>;
type Proof = usize;
type Hash = Vec<u8>;

#[derive(Debug)]
pub struct Timestamp(Duration);

impl ObjectHash for Timestamp {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "value" => &(self.0.as_secs()*(1e9 as u64) + self.0.subsec_nanos() as u64)
        )
    }
}

#[derive(Debug)]
pub struct HashOption(Option<Hash>);

impl ObjectHash for HashOption {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "value" => match self.0 {
                Some(v) => &v,
                None => &vec![]
            }
        )
    }
}

#[derive(Debug)]
pub struct Block {
    index: usize,
    timestamp: Timestamp,
    transactions: Vec<Transaction>,
    proof: Proof,
    previous_hash: HashOption
}

impl Block {
    pub fn new(index: usize, timestamp: Timestamp, transactions: Vec<Transaction>,
        proof: Proof, previous_hash: HashOption) -> Block
    {
        Block { index, timestamp, transactions, proof, previous_hash }
    }
}

impl ObjectHash for Block {
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

#[derive(Debug)]
pub struct BlockError;

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot add block")
    }
}

impl Error for BlockError {
    fn description(&self) -> &str {
        "Cannot add BLOCK"
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: usize
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: usize) -> Transaction {
        Transaction { sender, recipient, amount }
    }
}

impl ObjectHash for Transaction {
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

#[derive(Debug)]
pub struct TransactionError;

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot add transaction")
    }
}

impl Error for TransactionError {
    fn description(&self) -> &str {
        "Cannot add transaction"
    }
}

pub trait DistributedLedger {
    fn new_block(&mut self, timestamp: Timestamp, proof: Proof) -> Result<&LedgerRepr, BlockError>;
    fn add_transaction(&mut self, tx: Transaction) -> Result<usize, TransactionError>;
    fn last_block(&self) -> Option<&Block>;
    fn hash<T: ObjectHash>(obj: &T) -> Hash;
}

#[derive(Debug)]
pub struct StupidLedger {
    chain: LedgerRepr,
    block_txs: Vec<Transaction>
}

impl StupidLedger {
    pub fn new(chain: LedgerRepr) -> StupidLedger {
        StupidLedger { chain, block_txs: Vec::new() }
    }
}

impl DistributedLedger for StupidLedger {
    fn new_block(&mut self, timestamp: Timestamp, proof: Proof) -> Result<&LedgerRepr, BlockError> {
        let block = Block::new(
            self.chain.len() + 1,
            timestamp,
            self.block_txs.to_vec(),
            proof,
            if self.chain.len() == 0 {
                HashOption(Some(vec![0; 32]))
            } else {
                HashOption(Some(Self::hash(self.last_block().unwrap())))
            }
        );
        self.block_txs.clear();
        self.chain.push(block);
        Ok(&self.chain)
    }

    fn add_transaction(&mut self, tx: Transaction) -> Result<usize, TransactionError> {
        self.block_txs.push(tx);
        Ok(self.chain.len() + 1)
    }

    fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    fn hash<T: ObjectHash>(obj: &T) -> Hash {
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_transaction() {
        let sender = String::from("0x0001");
        let recipient = String::from("0x0002");
        let amount = 10000;
        let tx = Transaction::new(sender, recipient, amount);
        let mut stupid_chain = StupidLedger::new(Vec::new());
        assert_eq!(stupid_chain.add_transaction(tx.clone()).unwrap(), 1);
    }

    #[test]
    fn test_new_block() {
        let sender = String::from("0x0001");
        let recipient = String::from("0x0002");
        let amount = 10000;
        let tx = Transaction::new(sender, recipient, amount);
        let mut stupid_chain = StupidLedger::new(Vec::new());
        stupid_chain.add_transaction(tx.clone());

        assert_eq!(stupid_chain.new_block(12345, 1000).unwrap()[0],
            Block::new(1, 12345, vec![tx.clone()], 1000, None));
    }
}
