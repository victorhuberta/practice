//! # Utility
//!
//! Contains all utility functions.

use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Performs hex-related manipulations.
pub struct Hex;

impl Hex {
    /// Converts bytes to a hex string.
    ///
    /// # Examples
    ///
    /// ```
    /// use blockchain::ledger::util::Hex;
    ///
    /// let bytes = [0xff, 0x12, 0xa3];
    /// assert_eq!(&Hex::from_bytes(&bytes)[..], "ff12a3");
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> String {
        let mut s = String::new();
        for byte in bytes {
            write!(&mut s, "{:x}", byte).expect("Unable to write byte to string");
        }
        s
    }
}

#[cfg(test)]
mod hex_tests {
    use super::*;

    #[test]
    fn from_bytes() {
        let bytes = [0xff, 0x12, 0xa3];
        assert_eq!(&Hex::from_bytes(&bytes)[..], "ff12a3");
    }
}

/// Newtype for std::time::Duration.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Timestamp(pub Duration);

impl Timestamp {
    pub fn new(value: Duration) -> Timestamp {
        Timestamp(value)
    }

    pub fn current_nanos() -> Timestamp {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        Self::new(since_epoch)
    }
}

