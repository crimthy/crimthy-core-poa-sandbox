use crate::block::Block;
use crate::utils::replace;
pub mod message {
    
    pub enum MessageLabel {
        GetAllBlocks,
        SendBlock,
    }

    pub struct Message {
        blocks: Vec<super::Block>,
        label: MessageLabel,
        peers: Vec<String>,
    }
    impl Message {

        pub fn new(blocks: Vec<super::Block>, label: MessageLabel, peers: Vec<String>) -> Message {
            Message {
                blocks: blocks,
                label: label,
                peers: peers,
            }
        }

        pub fn get_label(&self) -> &MessageLabel {
            &self.label
        }

        pub fn get_blocks(&self) -> &Vec<super::Block> {
            &self.blocks
        }

        pub fn set_blocks(&mut self, blocks: Vec<super::Block>) {
            self.blocks = blocks;
        }
    }
}

use message::*;

pub fn broadcast_block(peers: &mut Vec<String>) {
    let mut new_peers = peers.clone();
    replace(&mut new_peers, &String::from("abc"), String::from("gga")).expect("Can't replace host address");
    println!("new peers : {:?}",new_peers);

    //let message = Message::new(
    //    vec![block],
    //    MessageLabel::SendBlock,
    //    peers,
    //);

}
