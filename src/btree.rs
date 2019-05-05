#[macro_use]
use bincode::{serialize, deserialize};
use super::row::*;
use super::cursor::*;
use super::table::*;

const LEAF_NODE_SIZE : u32 = std::mem::size_of::<LeafNode>() as u32;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NodeType {
    Leaf,
    Internal
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LeafNode {
    pub node_type: NodeType,
    pub num_cells: usize,
    pub keys: Vec<u32>,
    pub values: Vec<Row>,
}

impl LeafNode {

    pub fn new() -> LeafNode{
        return LeafNode{
            node_type: NodeType::Leaf,
            keys: Vec::new(),
            values: Vec::new(),
            num_cells: 0,
        }
    }

    pub fn key(&self, index: u32) -> Option<u32> {
        if let Some(key) = self.keys.get(index as usize) {
            return Some(*key)
        } else {
            return None
        }
    }

    pub fn value(&self, index: u32) -> Option<&Row> {
        return self.values.get(index as usize)
    }

    pub fn find(&self, key: u32) -> u32 {
        // let node = table.pager.get_page(page_num);
        let num_cells = self.num_cells;

        // Binary search
        let mut min_index : u32 = 0;
        let mut one_past_max_index = num_cells as u32;
        while one_past_max_index != min_index {
            let index = (min_index + one_past_max_index) / 2;
            let key_at_index = self.keys[index as usize];
            if key == key_at_index {
                return index;
            }

            if key < key_at_index {
                one_past_max_index = index;
            } else {
                min_index = index + 1;
            }
        }

        return min_index;
    }
}
