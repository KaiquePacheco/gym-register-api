mod dtos;

use dtos::{SignIn, SignUp};
use rocket::{
    form::{Form, Strict},
    post,
};

#[post("/signin", data = "<sign_in_form>")]
pub fn sign_in(sign_in_form: Form<Strict<SignIn<'_>>>) {}

#[post("/signup", data = "<sign_up_form>")]
pub fn sign_up(sign_up_form: Form<Strict<SignUp<'_>>>) {}
