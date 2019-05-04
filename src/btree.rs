#[macro_use]
use bincode::{serialize, deserialize};
use super::cursor::Cursor;
use super::row::*;

const LEAF_NODE_SIZE : u32 = std::mem::size_of::<LeafNode>() as u32;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LeafNode {
    pub num_cells: usize,
    pub keys: Vec<u32>,
    pub values: Vec<Row>,
}

impl LeafNode {

    pub fn new() -> LeafNode{
        return LeafNode{
            keys: Vec::new(),
            values: Vec::new(),
            num_cells: 0,
        }
    }

    pub fn key(self, index: u32) -> Option<u32> {
        if let Some(key) = self.keys.get(index as usize) {
            return Some(*key)
        } else {
            return None
        }
    }

    pub fn value(&self, index: u32) -> Option<&Row> {
        return self.values.get(index as usize)
    }

    // pub fn insert_key(&mut self, cell_num: usize, key: u32) {
    //     self.keys.insert(cell_num, key)
    // }

    // pub fn insert_value(&mut self, cell_num: usize, row: Row) {
    //     self.values.insert(cell_num, row)
    // }
}
