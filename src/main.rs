use anyhow::Result;
use postgres::{Client, NoTls};
mod db;

fn main() {
    let mut client = db::client();

    let query = "SELECT * FROM gittables_tables_info LIMIT 10";
    for row in client.query(query, &[]).unwrap() {
        println!("{:#?}", row);
    }
}
