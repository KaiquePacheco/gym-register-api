use std::error::Error;

use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;

use super::{
    super::users::{dtos::CreateUser, repository::UsersRepository},
    dtos::{forms::SignUp, token::TokenContent},
};

pub async fn register(
    user_repository: &UsersRepository,

    sign_up: &SignUp<'_>,
    key: &Hmac<Sha256>,
    cost: u32,
) -> Result<String, Box<dyn Error>> {
    let user = user_repository
        .create(CreateUser {
            email: sign_up.email,
            password: sign_up.password,
            username: sign_up.username,
            cost,
        })
        .await?;

    println!("{}", user.email);

    Ok(TokenContent::from(user).sign_with_key(key)?)
}
