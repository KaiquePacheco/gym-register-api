use super::super::users;
use super::custom_errs::sign_in::SignInError;
use super::dtos::{forms::SignIn, token::TokenContent};

use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;

use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection};

pub async fn sign_token<'r>(
    sign_in: &SignIn<'_>,
    key: &Hmac<Sha256>,

    conn: Object<AsyncPgConnection>,
) -> Result<String, SignInError> {
    let found_users = users::utils::find_by_email(conn, sign_in.email).await?;

    if found_users.is_empty() {
        return Err(SignInError::WrongEmail);
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

    Err(SignInError::WrongPassword)
}
