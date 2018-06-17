//! # Error
//!
//! Contains all error types used in ledger.

use std::error::Error;
use std::fmt;

/// Error type for Block-related failures.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockError;

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot add block")
    }
}

impl Error for BlockError {
    fn description(&self) -> &str {
        "Cannot add block"
    }
}

/// Error type for Transaction-related failures.
#[derive(Debug, Serialize, Deserialize)]
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
