use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: u64, data: &str, previous_hash: &str) -> Self {
        let hash = format!("{:x}", md5::compute(format!("{}{}{}{}", &index, &timestamp, &data, &previous_hash)));
        Block {
            index,
            timestamp,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
    current_index: u32,
}

impl Blockchain {
    fn new() -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::new(0, 0, "Genesis Block", "");
        chain.push(genesis_block);
        let current_index = 1;

        Blockchain { chain, current_index }
    }

    fn add_block(&mut self, data: &str) {
        let time_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let last_block = &self.chain[self.chain.len() - 1];
        let new_block = Block::new(self.current_index, time_since_epoch, &data, &last_block.hash);
        self.current_index += 1;
        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Some data for the first block");
    blockchain.add_block("Some data for the second block");
    blockchain.add_block("Some data for the third block");

    for block in blockchain.chain {
        println!("{:?}", block);
    }
}
