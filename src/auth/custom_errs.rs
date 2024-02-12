pub mod sign_in {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub enum SignInError {
        WrongEmail,
        WrongPassword,
    }

    impl Display for SignInError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::WrongEmail => write!(f, "wrong email"),
                Self::WrongPassword => write!(f, "wrong password"),
            }
        }
    }

    impl Error for SignInError {}
}
