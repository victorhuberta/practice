//! # Ledger
//!
//! Defines a general distributed ledger trait and everything else required by it.

pub mod error;
pub mod util;
pub mod example;

use std::time::Duration;
use self::error::*;

pub trait DistributedLedger<B: Block, T: Transaction> {
    /// Ledger representation
    type LedgerRepr;
    /// Proof used for block validations
    type Proof;

    fn new_block(&mut self, timestamp: Duration, proof: Self::Proof) -> Result<&Self::LedgerRepr, BlockError>;
    fn add_transaction(&mut self, tx: T) -> Result<usize, TransactionError>;
    fn last_block(&self) -> Option<&B>;
    fn hash(obj: &B) -> Vec<u8>;
}

pub trait Block {
    fn is_valid(&self) -> bool;
}

pub trait Transaction {
    fn is_valid(&self) -> bool;
}
