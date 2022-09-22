use anyhow::Result;
use postgres::{Client, NoTls};

/// returns (User, Password, Database)
fn get_credentials() -> Result<[String; 4]> {
    use std::env::var;

    Ok([
        var("DATABASE_USER")?,
        var("DATABASE_PASSWORD")?,
        var("DATABASE_DB")?,
        var("DATABASE_PORT").unwrap_or("5243".to_string()),
    ])
}

// example string:
//  "postgresql://dboperator:operatorpass123@localhost:5243/postgres"
fn get_config_str([user, password, database, port]: [String; 4]) -> String {
    format!("postgresql://{user}:{password}@localhost:{port}/{database}")
}

fn main() {
    let mut client = {
        let s = "postgresql://postgres:postgres@localhost/library";
        // let s = get_credentials()
        //     .map(get_config_str)
        //     .expect("to read credentials for database");

        match Client::connect(s, NoTls) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to connect to database: {e}");
                std::process::exit(1)
            }
        }
    };

    let query = "SELECT * FROM gittables_tables_info";
    for row in client.query(query, &[]).unwrap() {
        println!("{:#?}", row);
    }
}
