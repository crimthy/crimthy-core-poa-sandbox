//! Block data 
pub mod block_data {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct BlockData {
        timestamp: u128,
        data: i64,
    }

    impl BlockData {
        pub fn new(data: i64) -> BlockData {
            BlockData {
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Bad timestamp").as_nanos(),
                data: data,
            }
        }
        pub fn get_timestamp(&self) -> u128 {
            self.timestamp
        }

        pub fn get_data(&self) -> i64 {
            self.data
        }
    }
}

pub struct Block {
    data: block_data::BlockData,
    previous: String,
    current: String,
}


impl Block {

    pub fn new(data: i64, previous: String) -> Block {

        let data = block_data::BlockData::new(data);

        Block {
            data: data,
            previous: previous,
            current: String::from("a"),
        }
    }

    pub fn get_current(&self) -> &str {
        &self.current
    }

    pub fn get_content(&self) -> &block_data::BlockData {
        &self.data
    }
}
