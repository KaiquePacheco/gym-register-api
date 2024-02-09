pub mod token;
mod custom_errs;
mod dtos;

use std::ops::Deref;

use dtos::{SignIn, SignUp};
use rocket::{
    form::{Form, Strict},
    post,
    response::status,
    State,
};

use super::config::Configs;

use super::database::ConnectionPool;

#[post("/signin", data = "<sign_in_form>")]
pub async fn sign_in<'r>(
    pool: &State<ConnectionPool>,
    configs: &State<Configs>,

    sign_in_form: Form<Strict<SignIn<'_>>>,
) -> Result<status::Accepted<String>, status::BadRequest<&'r str>> {
    let conn = pool.get().await.unwrap();

    let sign_in_form = sign_in_form.deref().deref();
    let sign_token_result = token::sign_token(sign_in_form, &configs.jwt_key, conn).await;

    match sign_token_result {
        Ok(token) => Ok(status::Accepted(token)),
        Err(_) => Err(status::BadRequest("Wrong email or password")),
    }
}

#[post("/signup", data = "<sign_up_form>")]
pub async fn sign_up(sign_up_form: Form<Strict<SignUp<'_>>>) {}
