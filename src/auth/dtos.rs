pub mod forms {
    use super::super::validators::is_email;
    use rocket::form::FromForm;

    #[derive(FromForm)]
    pub struct SignIn<'r> {
        #[field(validate = is_email())]
        pub email: &'r str,
        pub password: &'r str,
    }

    #[derive(FromForm)]
    pub struct SignUp<'r> {
        pub username: &'r str,
        #[field(validate = is_email())]
        pub email: &'r str,
        pub password: &'r str,
    }
}

pub mod token {
    use rocket::serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::super::super::users::dtos::User;

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct TokenContent {
        pub user_id: Uuid,
        pub username: String,
        pub email: String,
    }

    impl From<User> for TokenContent {
        fn from(value: User) -> Self {
            Self {
                user_id: value.id,
                username: value.username,
                email: value.email,
            }
        }
    }
}
