use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

pub fn establish_connection() -> SqliteConnection {
    // Construct the path to the .env file
    let mut env_path = PathBuf::new();
    env_path.push(env::var("HOME").expect("HOME environment variable not set"));
    env_path.push(".local/share/cdf/.env"); // Adjust the path as necessary

    // Load the environment variables from the specified .env file
    dotenv::from_path(env_path.as_path()).ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
