pub struct App {
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

// simplistic basis for mining scheme, when mining a block, data is hashed for block and hash is found
// which starts with two zeros
const DIFFICULTY_PREFIX: &str = "00";

// binary representation of a given byte array in form of String
// used to check whether hash fits the DIFFICULTY_PREFIX condition
fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    // creates the first, hard-coded, block in blockchain
    // special block, which is the start of the blockchain
    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: "genesis".to_string(),
            data: "genesis".to_string(),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43",
        };
        self.blocks.push(genesis_block);
    }
    // gets last block in the chain, validate if block is valid and can be added
    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is atleast a single block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("invalid block");
        }
    }
    // validating block logic
    // ensures blockchain adheres to chain property & is hard to tamper with
    // 
    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block id: {}, has an invalid previous hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can't decode from hex");
        ).starts_with(DIFFICULTY_PREFIX) 
        {
            warn!("block id: {}, has an invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block id: {}, is not the block after the latest: {}", block.id, previous_block.id
            );
            return false;
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block id: {}, has an invalid hash", block.id);
            return false;
        }
        true
    }
    // validating a whole chain
    fn is_chain_valid(&mut self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.block.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    // chooses which chain to use
    fn choose_chain(&mut self, local: Vec, remote: Vec) -> Vec {
        // always choose the longest valid chain
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("local & remote chains invalid");
        }
    }
}

// implementation of mining scheme
// when new block created, mine_block is called, which returns nonce & a hash

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);

        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        info!("Mining block...");
        let mut nonce = 0;

        loop {
            if nonce & 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
            let binary_hash = hash_to_binary_representation(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                info!(
                    "block mined! nonce: {}, hash: {}, binary hash: {}",
                    nonce,
                    hex::encode(&hash),
                    binary_hash
                );
                return (nonce, hex::encode(&hash));
            }
            nonce += 1;
        }
    }
}

