use super::table::Table;
use super::row::Row;
use super::constants::*;

pub struct Cursor<'a> {
    pub table: &'a mut Table,
    pub row_num: u32,
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
            let slot = self.value();
            let pager = &self.table.pager;
            let pages = &pager.pages;
            let row = &pages[slot.page][slot.page_index];
            f(row);
            self.advance();
        }
    }

    pub fn value(&mut self) -> Slot {
        let row_num = self.row_num;
        let page_num = row_num / ROWS_PER_PAGE;
        self.table.pager.ensure_page(page_num);
        let page_index = row_num % ROWS_PER_PAGE;
        return Slot{
            page_index: page_index as usize,
            page: page_num as usize,
        }
    }

    pub fn advance(&mut self) {
        self.row_num += 1;
        if self.row_num >= self.table.num_rows {
            self.end_of_table = true
        }
    }
}