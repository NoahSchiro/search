use std::fs;

/*
A file contains multiple "chunks" where
each chunk is a line number, this makes
it more searchable.
*/
pub struct File {
    pub full_path: String,
    pub contents: Vec<String> 
}

pub fn register_file(full_path: String) -> File {

    let contents: Vec<String> = fs::read_to_string(full_path.clone()) // Read file contents
        .expect("Should have been able to read file") // Handle error
        .split("\n") // Split on the new line
        .map(|x| x.to_string()) // Map each line to a string
        .collect(); // Collect into vector
 
    File {
        full_path,
        contents 
    }
}


