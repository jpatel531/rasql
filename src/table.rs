use super::pager::Pager;
use super::cursor::Cursor;
use super::constants::*;

pub struct Table {
    pub pager: Pager,
    pub num_rows: u32,
}

impl Table {
    pub fn start(&mut self) -> Cursor {
        let end_of_table = self.num_rows == 0;
        return Cursor{
            table: self,
            row_num: 0,
            end_of_table: end_of_table,
        }
    }

    pub fn end(&mut self) -> Cursor {
        let row_num = self.num_rows;
        return Cursor{
            table: self,
            row_num: row_num,
            end_of_table: true,
        }
    }

    pub fn close(&mut self) {
        let num_full_pages : usize = self.num_rows as usize / ROWS_PER_PAGE as usize;

        for num in 0..num_full_pages {
            self.pager.flush(num, PAGE_SIZE as usize);
        }

        let num_additional_rows = self.num_rows % ROWS_PER_PAGE;
        if num_additional_rows > 0 {
            let page_num = num_full_pages;
            if self.pager.pages.get(page_num).is_some() {
                self.pager.flush(page_num, num_additional_rows as usize * ROW_SIZE as usize);
            }
        }
    }
}
