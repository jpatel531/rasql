use super::row::*;

pub const PAGE_SIZE : u32 = 4096;
pub const ROW_SIZE : u32 = std::mem::size_of::<Row>() as u32;
pub const TABLE_MAX_PAGES : u32 = 100;
pub const ROWS_PER_PAGE : u32 = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS : u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;
