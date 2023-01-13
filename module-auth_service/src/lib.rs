#![allow(dead_code, unused_variables)]

// A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

mod database;

mod auth_utils;


pub use auth_utils::models::Credentials;
pub use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
