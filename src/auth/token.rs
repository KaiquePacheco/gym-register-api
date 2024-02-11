use std::error::Error;

use super::{
    super::users,
    custom_errs::sign_in::SignInError,
    dtos::{forms::SignIn, token::TokenContent},
};

use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;

use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection};

pub async fn sign_token<'r>(
    sign_in: &SignIn<'_>,
    key: &Hmac<Sha256>,

    conn: Object<AsyncPgConnection>,
) -> Result<String, Box<dyn Error>> {
    let found_users = users::utils::find_by_email(conn, sign_in.email).await?;

    if found_users.is_empty() {
        return Err(Box::new(SignInError::WrongEmail));
    }

    for data in found_users {
        if !bcrypt::verify(sign_in.password, &data.password_hash)? {
            continue;
        }

        match TokenContent::from(data).sign_with_key(key) {
            Ok(token) => return Ok(token),
            Err(_) => (),
        }
    }

    Err(Box::new(SignInError::WrongPassword))
}
