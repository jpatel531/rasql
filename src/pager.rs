use std::io::{Write};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;
use std::fs::OpenOptions;

use super::row::*;
use super::constants::*;
use super::btree::*;

pub struct Pager {
    pub file: File,
    pub pages: Vec<LeafNode>,
    pub num_pages: usize,
}

impl Pager {
    pub fn open(filename: String) -> Result<Pager, String> {
        let file_options = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename);

        match file_options {
            Ok(mut file) => {
                file.seek(SeekFrom::End(0)).unwrap();

                let file_length = file.metadata().unwrap().len() as u32;
                let num_pages = file_length / PAGE_SIZE;
                if file_length % PAGE_SIZE != 0 {
                    return Err(String::from("DB file is not a whole number of pages. Corrupt file"));
                }

                let mut pager = Pager{
                    file: file,
                    pages:  Vec::with_capacity(TABLE_MAX_PAGES as usize),
                    num_pages: num_pages as usize,
                };

                if num_pages == 0 {
                    pager.get_page(0);
                }

                return Ok(pager)
            },
            Err(err) => {
                return Err(format!("{}", err))
            }
        }
    }

    pub fn get_page(&mut self, page_num: u32) -> &mut LeafNode {
        if page_num > TABLE_MAX_PAGES {
            println!("Tried to fetch page number out of bounds. {} > {}", page_num, TABLE_MAX_PAGES);
            std::process::exit(1);
        }

        if self.pages.get(page_num as usize).is_none() {
            self.create_page(page_num);
        }

        return &mut self.pages[page_num as usize];
    }

    fn create_page(&mut self, page_num: u32){
        let mut file = BufReader::new(&self.file);

        let file_length = self.file.metadata().unwrap().len();

        let mut num_pages = file_length / PAGE_SIZE as u64;
        if file_length % PAGE_SIZE as u64 > 0 {
            num_pages += 1;
        }

        let mut page = LeafNode::new();
        if page_num as u64 <= num_pages {
            file.seek(SeekFrom::Start((page_num*PAGE_SIZE as u32) as u64)).unwrap();
            let mut buf = [0; PAGE_SIZE as usize];
            file.read(&mut buf).unwrap();
            page = bincode::deserialize(&buf).unwrap();
        }
        self.pages.insert(page_num as usize, page);
        if page_num >= self.num_pages as u32 {
            self.num_pages += 1;
        }
    }

    pub fn flush(&mut self, page_num: usize) {
        let page = &self.pages[page_num];
        self.file.seek(SeekFrom::Start((page_num*(PAGE_SIZE as usize)) as u64)).unwrap();
        let mut buf = bincode::serialize(&page).unwrap();
        buf.resize(PAGE_SIZE as usize, 0);
        self.file.write(&buf).unwrap();
    }
}
