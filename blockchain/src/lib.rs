extern crate sha3;

use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

type LedgerRepr = Vec<Block>;
type Proof = usize;
type Hash = [u8; 32];

#[derive(Debug)]
pub struct Block {
    index: usize,
    timestamp: Duration,
    transactions: Vec<Transaction>,
    proof: Proof,
    previous_hash: Hash
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
    fn add_block(&mut self, block: Block) -> Result<&LedgerRepr, BlockError>;
    fn add_transaction(&mut self, tx: Transaction) -> Result<usize, TransactionError>;
    fn last_block(&self) -> Option<&Block>;
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
    fn add_block(&mut self, block: Block) -> Result<&LedgerRepr, BlockError> {
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
}
