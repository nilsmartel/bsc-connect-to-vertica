use anyhow::Result;
use postgres::{fallible_iterator::FallibleIterator, Client, NoTls};

/// returns (User, Password, Database)
fn get_credentials() -> Result<(String, String, String)> {
    use std::env::var;

    Ok((
        var("DATABASE_USER")?,
        var("DATABASE_PASSWORD")?,
        var("DATABASE_DB")?,
    ))
}

// example string:
//  "postgresql://dboperator:operatorpass123@localhost:5243/postgres"
fn get_config_str((user, password, database): (String, String, String)) -> String {
    format!("postgresql://{user}:{password}@localhost:5243/{database}")
}

fn main() {
    let mut client = {
        let s = get_credentials()
            .map(get_config_str)
            .expect("to read credentials for database");

        Client::connect(&s, NoTls).expect("to make postgres connection")
    };

    let query = "SELECT * FROM gittables_tables_info";
    for row in client.query(query, &[]).unwrap() {
        println!("{:#?}", row);
    }
}
