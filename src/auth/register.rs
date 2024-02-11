use std::error::Error;

use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection};
use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;

use super::{
    super::users::utils::create,
    dtos::{forms::SignUp, token::TokenContent},
};

pub async fn register(
    sign_up: &SignUp<'_>,
    key: &Hmac<Sha256>,
    cost: u32,

    conn: Object<AsyncPgConnection>,
) -> Result<String, Box<dyn Error>> {
    let user = create(
        conn,
        sign_up.username.into(),
        sign_up.email.into(),
        sign_up.password.into(),
        cost,
    )
    .await?;

    println!("{}", user.email);

    Ok(TokenContent::from(user).sign_with_key(key)?)
}
