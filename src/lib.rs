#![allow(dead_code, unused_variables)]

//ref:https://www.youtube.com/watch?v=969j0qnJGi8&t=406s

mod auth_utils;
mod database;

use auth_utils::model::Credentials;
use auth_utils::*;
use database::*;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}
