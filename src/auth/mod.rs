mod custom_errs;
mod dtos;
pub mod register;
pub mod token;
pub mod validators;

use dtos::forms::{SignIn, SignUp};
use rocket::{
    form::{Form, Strict},
    http::Status,
    post,
    response::status,
    State,
};

use super::{config::Configs, users::repository::UsersRepository};

#[post("/signin", data = "<sign_in_form>")]
pub async fn sign_in<'r>(
    user_repository: &State<UsersRepository>,
    configs: &State<Configs>,

    sign_in_form: Form<Strict<SignIn<'_>>>,
) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let sign_in_result =
        token::sign_token(&(**user_repository), &(**sign_in_form), &configs.jwt_key).await;

    match sign_in_result {
        Ok(token) => Ok(status::Accepted(token)),
        Err(e) => Err(status::BadRequest(e.to_string())),
    }
}

#[post("/signup", data = "<sign_up_form>")]
pub async fn sign_up(
    user_repository: &State<UsersRepository>,
    configs: &State<Configs>,

    sign_up_form: Form<Strict<SignUp<'_>>>,
) -> Result<status::Accepted<String>, status::Custom<String>> {
    let create_user_result = register::register(
        &(**user_repository),
        &(**sign_up_form),
        &configs.jwt_key,
        configs.bcrypt_cost,
    )
    .await;

    match create_user_result {
        Ok(token) => Ok(status::Accepted(token)),
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}
