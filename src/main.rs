mod database;
mod file;
mod result;

use crate::database::{Database, register_database};
use crate::result::SearchResult;

fn main() {

    //Build the database
    let db: Database = register_database(String::from("/home/noah/Work/search/database/"));

    let search_query = String::from("feel");

    //Query the database
    let results: Vec<SearchResult> = db.search(search_query, 5);

    //Print out the result
    for result in results {
        println!("{}", result);
    }
}
