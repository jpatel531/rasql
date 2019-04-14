use super::pager::Pager;
use super::cursor::Cursor;

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
        self.pager.flush();
    }
}
