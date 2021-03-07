use super::*;

/// Test the successful creation of a new block.
#[test]
fn new_block() {
    let mut block = Block::new("Hello world");
    block.hash = block.calculate_hash();
    assert_eq!(block.data, "Hello world");
    assert_eq!(block.hash, block.calculate_hash());
    block.data = "Changed data";
    assert_ne!(block.hash, block.calculate_hash());
}

/// Test the successful creation of a new blockchain.
#[test]
fn new_blockchain() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.0.len(), 1);
    assert_eq!(blockchain.0[0].data, "Genesis block");
}

/// Test the detection of a valid blockchain as valid.
#[test]
fn verify_blockchain() {
    let mut blockchain = Blockchain::new();
    blockchain.push(Block::new("2. Block"));
    blockchain.push(Block::new("3. Block"));
    println!("{:#?}", blockchain);
    assert_eq!(blockchain.verify(), Ok(()));
}

/// Test the detection of a changed block.
///
/// The idea is to change the data of a block. The blockchain should detect that
/// the saved hash does not match to a newly computed one.
#[test]
fn blockchain_verify_inside_block() {
    let mut blockchain = Blockchain::new();
    blockchain.push(Block::new("2. Block"));
    blockchain.push(Block::new("3. Block"));
    blockchain.0[1].data = "Changed something";
    assert_eq!(blockchain.verify(), Err(VerifyError::InsideBlock(1)));
}

/// Test the detection of a changed block with recomputed hash.
///
/// The idea is to change the data of a block, recompute its hash and save
/// it into the block. So the block on its own seems to be valid. This test
/// checks the ability of the blockchain to detect the now arisen broken relationship
/// with the following block.
#[test]
fn blockchain_verify_previous_block_relationship() {
    let mut blockchain = Blockchain::new();
    blockchain.push(Block::new("2. Block"));
    blockchain.push(Block::new("3. Block"));
    blockchain.0[1].data = "Changed something";
    blockchain.0[1].hash = blockchain.0[1].calculate_hash();
    assert_eq!(blockchain.verify(), Err(VerifyError::PreviousBlockRelationship(2)));
}
