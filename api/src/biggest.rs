use serde::{Deserialize, Serialize};
use std::cmp::Reverse;

#[derive(Deserialize, Serialize, Clone)]
pub struct DocInfo {
    pub path: String,
    pub size: usize,
}

// a data structure to keep only the n biggest inserted files
pub struct Biggest {
    values: Vec<DocInfo>,
    min_size: usize,
    keep_count: usize,
    sort_count: usize,
}

impl Biggest {
    pub fn new(keep_count: usize) -> Biggest {
        assert!(keep_count > 0);
        Biggest {
            values: Vec::new(),
            min_size: 0,
            keep_count,
            sort_count: 0,
        }
    }

    fn sort(&mut self) {
        self.sort_count += 1;
        self.values
            .sort_by(|a, b| Reverse(a.size).cmp(&Reverse(b.size)));
    }

    pub fn insert(&mut self, doc: DocInfo) {
        if self.values.len() < self.keep_count {
            if doc.size < self.min_size {
                self.min_size = doc.size;
            }

            self.values.push(doc);
        } else if doc.size > self.min_size {
            self.values.push(doc);
            self.sort();
            self.values.pop();
            self.min_size = self.values.last().unwrap().size;
        }
    }

    // is this document big enough to be in the n biggest docs?
    pub fn is_big_enough(&self, size: usize) -> bool {
        size > self.min_size
    }

    pub fn values(&self) -> Vec<DocInfo> {
        self.values.to_owned()
    }

    pub fn sort_count(&self) -> usize {
        self.sort_count
    }
}
