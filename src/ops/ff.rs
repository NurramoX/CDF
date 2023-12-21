use crate::models::{KeyListItem, KeyPath};
use crate::db::establish_connection;
use diesel::prelude::*;

use diesel::result::Error as DieselError;
use std::io::{self, Write};

pub fn create_new_key_path(k: &str, p: &str) {
    println!("Creating new key '{}' with path '{}'", k, p);
    use crate::schema::fast_forward::dsl::*;
    let mut connection = establish_connection();

    let new_path = KeyPath {
        key: k,
        path: p,
    };

    match diesel::insert_into(fast_forward)
        .values(&new_path)
        .execute(&mut connection) {
        Ok(_) => print!("Key '{}' successfully created.", k),
        Err(DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            print!("Key '{}' already exists. Do you want to overwrite it? (yes/no)\t", k);
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            if user_input.trim().to_lowercase().starts_with('y') {
                update_key_path(k, p);
            } else {
                println!("Aborted updating key '{}'.", k);
            }
        },
        Err(e) => {
            println!("Error saving new key path: {:?}", e);
        }
    }
}

pub fn update_key_path(k: &str, p: &str) {
    println!("Updating path for key '{}' to '{}'", k, p);
    use crate::schema::fast_forward::dsl::*;
    let mut connection = establish_connection();

    match diesel::update(fast_forward.filter(key.eq(k)))
        .set(path.eq(p))
        .execute(&mut connection) {
        Ok(_) => println!("Key '{}' successfully updated", k),
        Err(e) => println!("Error updating key path: {:?}", e),
    }
}

pub fn get_path_from_key(k: &String) {
    use crate::schema::fast_forward::dsl::*;
    let mut connection = establish_connection();

    let result = fast_forward
        .filter(key.eq(k))
        .select(path)
        .first::<String>(&mut connection);

    match result {
        Ok(p) => println!("{}", p),
        Err(e) => println!("Could not find path for key '{}': {:?}", k, e),
    }
}

pub fn delete_key(k: &String) {
    use crate::schema::fast_forward::dsl::*;
    let mut connection = establish_connection();

    match diesel::delete(fast_forward.filter(key.eq(k))).execute(&mut connection) {
        Ok(_) => println!("Key-path pair with key '{}' has been deleted", k),
        Err(e) => println!("Error deleting key-path pair with key '{}': {:?}", k, e),
    }
}

pub fn list_keys(substring: &str) {
    use crate::schema::fast_forward::dsl::*;
    let mut connection = establish_connection();

    let results = fast_forward
        .filter(key.like(format!("%{}%", substring)))
        .select(key)
        .load::<Option<String>>(&mut connection);

    match results {
        Ok(keys) => {
            // Listing all keys containing the substring
            for k in keys {
                if let Some(key_value) = k {
                    println!("{}", key_value);
                }
            }
        }
        Err(e) => println!("Error listing keys: {:?}", e),
    }
}
