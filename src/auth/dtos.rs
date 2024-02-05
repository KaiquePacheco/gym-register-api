use rocket::form::FromForm;

#[derive(FromForm)]
pub struct SignIn<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(FromForm)]
pub struct SignUp<'r> {
    username: &'r str,
    email: &'r str,
    password: &'r str,
}
