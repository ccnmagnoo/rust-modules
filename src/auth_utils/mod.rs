use crate::database::get_user;
use model::Credentials;

pub fn login(creds: Credentials) {
    //authenticate...
    get_user();
}
pub fn logout() {
    //log user out...
}

pub mod model;
