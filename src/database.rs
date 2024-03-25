use std::fs;

use regex::Regex;

use crate::file::{File, register_file};
use crate::result::SearchResult;

pub struct Database {
    pub contents: Vec<File>
}

impl Database {

    #[allow(dead_code)]
    pub fn len(&self) -> usize { self.contents.len() }

    //Text contains search query
    pub fn search(&self, query: String, topk: usize) -> Vec<SearchResult> {

        let re = Regex::new(
            format!(".*{}.*", query.as_str()).as_str()
        ).unwrap();

        let mut result: Vec<SearchResult> = Vec::new();

        for file in &self.contents {
            for (idx, line) in file.contents.iter().enumerate() {

                //If regex matches, return
                if re.captures(line).is_some() {
                    result.push(
                        SearchResult {
                            file: file.full_path.clone(),
                            line_num: idx,
                            content: line.to_string().clone()
                        }
                    );

                    if result.len() >= topk {
                        return result
                    }
                }
            }
        }

        result
    }
}

pub fn register_database(dir: String) -> Database {

    let paths = fs::read_dir(dir).unwrap();

    let files: Vec<File> = paths
        .map(|path| register_file(path.unwrap().path().display().to_string()))
        .collect();

    Database {
        contents: files
    }
}
