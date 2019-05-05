use super::table::Table;
use super::row::Row;
use super::constants::*;
use super::pager::*;
use super::btree::*;

pub struct Cursor<'a> {
    pub page: &'a mut LeafNode,
    pub page_num: u32,
    pub cell_num: u32,
    pub end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn iterate<F>(&mut self, f: F) where
    F: Fn(&Row) {
        while !self.end_of_table {
            let row = self.value();
            f(&row);
            self.advance();
        }
    }

    pub fn value(&mut self) -> &Row {
        return self.page.value(self.cell_num).unwrap()
    }

    pub fn advance(&mut self) {
        self.cell_num += 1;
        if self.cell_num >= self.page.num_cells as u32 {
            self.end_of_table = true
        }
    }

    pub fn insert_leaf_node(&mut self, key: u32, value: Row) {
        // TODO: limit maximum
        self.page.keys.insert(self.cell_num as usize, key);
        self.page.values.insert(self.cell_num as usize, value);
        self.page.num_cells += 1;
    }
}