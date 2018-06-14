extern crate sha3;

use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

type Chain = Vec<Block>;
type Proof = usize;
type Hash = [u8; 32];

pub struct Block {
    index: usize,
    timestamp: Duration,
    transactions: Vec<Transaction>,
    proof: Proof,
    previous_hash: Hash
}

pub struct Transaction {
    sender: String,
    recipient: String,
    amount: usize
}

impl Transaction {
    fn new(sender: String, recipient: String, amount: usize) -> Transaction {
        Transaction { sender, recipient, amount }
    }
}

#[derive(Debug)]
pub struct ChainError;

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failure in manipulating chain")
    }
}

impl Error for ChainError {
    fn description(&self) -> &str {
        "Failure in manipulating chain"
    }
}

pub trait DistributedLedger {
    fn add_block(&mut self, block: Block) -> Result<&Chain, ChainError>;
    fn add_transaction(&mut self, tx: Transaction) -> Result<usize, ChainError>;
    fn last_block(&self) -> Option<&Block>;
}

struct StupidLedger {
    content: Chain,
    block_txs: Vec<Transaction>
}

impl StupidLedger {
    fn new(content: Chain) -> StupidLedger {
        StupidLedger { content, block_txs: Vec::new() }
    }
}

impl DistributedLedger for StupidLedger {
    fn add_block(&mut self, block: Block) -> Result<&Chain, ChainError> {
        self.content.push(block);
        Ok(&self.content)
    }

    fn add_transaction(&mut self, tx: Transaction) -> Result<usize, ChainError> {
        self.block_txs.push(tx);
        Ok(self.last_block().unwrap().index + 1)
    }

    fn last_block(&self) -> Option<&Block> {
        self.content.last()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_transaction() {
        let sender = "0x0001";
        let recipient = "0x0002";
        let amount = 10000;
        let tx = Transaction::new(sender, recipient, amount);
        let stupid_chain = StupidLedger::new(Vec::new());
        stupid_chain.add_transaction(tx);
        assert_eq!(stupid_chain.last_block().unwrap().transactions[0], tx);
    }
}
