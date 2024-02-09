use super::dtos::{SignIn, TokenContent};
use super::custom_errs::SignInError;
use super::super::users::{self, dtos::UserData};

use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;

use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection};

use std::error::Error;

pub async fn sign_token(
    sign_in: &SignIn<'_>,
    key: &Hmac<Sha256>,

    conn: Object<AsyncPgConnection>,
) -> Result<String, Box<dyn Error>> {
    let user_datas = users::utils::find_by_email(conn, sign_in.email).await?;

    for data in user_datas {
        if !bcrypt::verify(sign_in.password, &data.password_hash)? {
            continue;
        }
        match TokenContent::from(data).sign_with_key(key) {
            Ok(token) => return Ok(token),
            Err(_) => (),
        }
    }

    return Err(Box::new(SignInError {}));
}

impl From<UserData> for TokenContent {
    fn from(value: UserData) -> Self {
        Self {
            user_id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}
