use super::pager::Pager;
use super::cursor::Cursor;
use super::btree::*;

pub struct Table {
    pub pager: Pager,
    pub root_page_num: u32,
}

impl Table {
    pub fn start(&mut self) -> Cursor {
        let root_page_num = self.root_page_num;
        let pager = &mut self.pager;
        let root_node = pager.get_page(root_page_num);

        let end_of_table = root_node.num_cells == 0;
        return Cursor{
            page: root_node,
            page_num: root_page_num,
            cell_num: 0,
            end_of_table: end_of_table,
        }
    }

    pub fn find(&mut self, page_num: u32, key: u32) -> Cursor {
        let node = self.pager.get_page(page_num);

        if node.node_type != NodeType::Leaf {
            panic!("need to implement internal nodes")
        }

        let cell_num = node.find(key);
        return Cursor{
            page: node,
            page_num: page_num,
            cell_num: cell_num,
            end_of_table: false
        }
    }

    pub fn close(&mut self) {
        let num_pages : usize = self.pager.pages.len();
        for num in 0..num_pages {
            self.pager.flush(num);
        }
    }
}
