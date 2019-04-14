use std::io::{Write};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;
use std::fs::OpenOptions;

use super::constants::*;
use super::row::*;

type Page = Vec<Row>;

pub struct Pager {
    pub file: File,
    pub pages: Vec<Page>
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
                return Ok(Pager{
                    file: file,
                    pages: Vec::with_capacity(TABLE_MAX_PAGES as usize),
                })
            },
            Err(err) => {
                return Err(format!("{}", err))
            }
        }
    }

    pub fn ensure_page(&mut self, page_num: u32) {
        if page_num > TABLE_MAX_PAGES {
            println!("Tried to fetch page number out of bounds. {} > {}", page_num, TABLE_MAX_PAGES);
            std::process::exit(1);
        }

        if self.pages.get(page_num as usize).is_none() {
            // Cache miss, load from file
            let mut file = BufReader::new(&self.file);

            file.seek(SeekFrom::Start((page_num*PAGE_SIZE as u32) as u64)).unwrap();

            let mut buf = [0; PAGE_SIZE as usize];
            file.read(&mut buf).unwrap();
            let page : Page = bincode::deserialize(&buf).unwrap();
            self.pages.insert(page_num as usize, page);
        }
    }

    pub fn flush(&mut self) {
        let pages = &self.pages;
        for (num, page) in pages.iter().enumerate() {
            self.file.seek(SeekFrom::Start((num*PAGE_SIZE as usize) as u64)).unwrap();
            let mut buf = bincode::serialize(&page).unwrap();
            buf.resize(page.len() * ROW_SIZE as usize, 0);
            self.file.write(&buf).unwrap();
        }
    }
}
