use super::table::Table;
use super::row::Row;
use super::constants::*;

pub struct Cursor<'a> {
    pub table: &'a mut Table,
    pub page_num: u32,
    pub cell_num: u32,
    pub end_of_table: bool,
}

#[derive(Debug)]
pub struct Slot {
    pub page: usize,
    pub page_index: usize,
}

impl<'a> Cursor<'a> {
    pub fn iterate<F>(&'a mut self, f: F) where
    F: Fn(&Row) {
        while !self.end_of_table {
            let row = self.value();
            f(&row);
            self.advance();
        }
    }

    pub fn value(&mut self) -> &Row {
        let node = self.table.pager.get_page(self.page_num);
        return node.value(self.cell_num).unwrap()
    }

    pub fn advance(&mut self) {
        let node = self.table.pager.get_page(self.page_num);
        self.cell_num += 1;
        if self.cell_num >= node.num_cells as u32 {
            self.end_of_table = true
        }
    }

    pub fn insert_leaf_node(&mut self, key: u32, value: Row) {
        // TODO: limit maximum
        let mut node = self.table.pager.get_page(self.page_num);
        node.keys.insert(self.cell_num as usize, key);
        node.values.insert(self.cell_num as usize, value);
        node.num_cells += 1;
    }
}