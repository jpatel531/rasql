use super::pager::Pager;
use super::cursor::Cursor;
use super::constants::*;

pub struct Table {
    pub pager: Pager,
    pub root_page_num: u32,
}

impl Table {
    pub fn start(&mut self) -> Cursor {
        let root_page_num = self.root_page_num;
        let root_node = self.pager.get_page(root_page_num);

        let end_of_table = root_node.num_cells == 0;
        return Cursor{
            table: self,
            page_num: root_page_num,
            cell_num: 0,
            end_of_table: end_of_table,
        }
    }

    pub fn end(&mut self) -> Cursor {
        let root_page_num = self.root_page_num;
        let root_node = self.pager.get_page(root_page_num);
        return Cursor{
            page_num: root_page_num,
            cell_num: root_node.num_cells as u32,
            end_of_table: true,
            table: self,
        }
    }

    pub fn close(&mut self) {
        let num_pages : usize = self.pager.pages.len();
        for num in 0..num_pages {
            self.pager.flush(num);
        }
    }
}
