use std::error::Error;

use super::{
    super::users::repository::UsersRepository,
    custom_errs::sign_in::SignInError,
    dtos::{forms::SignIn, token::TokenContent},
};

use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;

pub async fn sign_token(
    user_repository: &UsersRepository,

    sign_in: &SignIn<'_>,
    key: &Hmac<Sha256>,
) -> Result<String, Box<dyn Error>> {
    let found_users = user_repository.find_by_email(sign_in.email).await?;

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
