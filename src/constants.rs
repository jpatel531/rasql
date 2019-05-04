use super::row::*;

pub const PAGE_SIZE : u32 = 4096;
pub const ROW_SIZE : u32 = std::mem::size_of::<Row>() as u32;
pub const TABLE_MAX_PAGES : u32 = 100;

