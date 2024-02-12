use rocket::form::{self, Error};

pub fn is_email<'r>(email: &&'r str) -> form::Result<'r, ()> {
    if email.chars().find(|&char| char.clone() == '@').is_some() {
        return Ok(());
    }

    Err(Error::validation("invalid email"))?
}
