//! A test implementation of a very bad blockchain.

extern crate chrono;

use chrono::{Local, DateTime};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// A block of a blockchain.
#[derive(Debug)]
pub struct Block<'a> {
    /// The time on which this block was created.
    timestamp: DateTime<Local>,
    /// The hash of this block.
    pub hash: u64,
    /// The hash of the previous block in the blockchain.
    pub previous_block_hash: u64,
    /// The data of this block.
    pub data: &'a str,
}

impl<'a> Block<'a> {
    /// Creates a new block with the data and the current timestamp.
    ///
    /// # Examples
    /// ```
    /// use blockchain::Block;
    ///
    /// let block = Block::new("Block data");
    /// assert_eq!(block.data, "Block data");
    /// assert_eq!(block.hash, 0);
    /// assert_eq!(block.previous_block_hash, 0);
    /// ```
    pub fn new(data: &'a str) -> Self {
        Self {
            timestamp: Local::now(),
            hash: 0,
            previous_block_hash: 0,
            data,
        }
    }

    /// Calculates the hash value of this block.
    /// See the [`Hash`](`Self::hash()`) implementation for details.
    fn calculate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for Block<'_> {
    /// Hashes the current block and includes `data`, `timestamp` and
    /// the `previous_block_hash`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.timestamp.hash(state);
        self.previous_block_hash.hash(state);
    }
}

pub struct Blockchain<'a>(Vec<Block<'a>>);

/// An error verifying the blockchain.
#[derive(PartialEq, Debug)]
enum VerifyError {
    /// A corrupted block specified by its index in the blockchain.
    InsideBlock(usize),
    /// A corrupted relationship between a block and its previous block.
    /// The index specifies the corrupted block.
    PreviousBlockRelationship(usize),
}

impl<'a> Blockchain<'a> {
    /// Creates a blockchain with a genesis block.
    ///
    /// # Examples
    /// ```
    /// use blockchain::Blockchain;
    ///
    /// let blockchain = Blockchain::new();
    /// assert_eq!(blockchain.len(), 1);
    /// ```
    pub fn new() -> Self {
        let mut genesis_block = Block::new("Genesis block");
        genesis_block.hash = genesis_block.calculate_hash();
        Self { 0: vec![genesis_block] }
    }

    /// Pushes a new block onto the blockchain.
    ///
    /// # Examples
    /// ```
    /// use blockchain::{Block, Blockchain};
    ///
    /// let mut blockchain = Blockchain::new();
    /// assert_eq!(blockchain.len(), 1); // Genesis block
    /// blockchain.push(Block::new("First data block"));
    /// assert_eq!(blockchain.len(), 2);
    /// ```
    pub fn push(&mut self, mut block: Block<'a>) {
        block.previous_block_hash = self.0.last().unwrap().hash;
        block.hash = block.calculate_hash();
        self.0.push(block);
    }

    /// Returns the number of blocks in the blockchain.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks the integrity of its blockchain by checking for
    /// valid blocks and valid relationships between blocks.
    fn verify(&self) -> Result<(), VerifyError> {
        for i in 1..self.0.len() {
            if self.0[i].hash != self.0[i].calculate_hash() {
                return Err(VerifyError::InsideBlock(i));
            }
            if self.0[i].previous_block_hash != self.0[i-1].hash {
                return Err(VerifyError::PreviousBlockRelationship(i));
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Blockchain<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests;
