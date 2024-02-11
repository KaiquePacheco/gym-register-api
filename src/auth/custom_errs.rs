pub mod sign_in {
    use std::{error::Error, fmt::Display};

    use bcrypt::BcryptError;
    use diesel::result::Error as DieselError;

    #[derive(Debug)]
    pub enum SignInError {
        WrongEmail,
        WrongPassword,
        Diesel(DieselError),
        Bcrypt(BcryptError),
    }

    impl From<DieselError> for SignInError {
        fn from(value: DieselError) -> Self {
            Self::Diesel(value)
        }
    }
    impl From<BcryptError> for SignInError {
        fn from(value: BcryptError) -> Self {
            Self::Bcrypt(value)
        }
    }

    impl Display for SignInError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::WrongEmail => write!(f, "wrong email"),
                Self::WrongPassword => write!(f, "wrong password"),
                Self::Bcrypt(e) => write!(f, "{e}"),
                Self::Diesel(e) => write!(f, "{e}"),
            }
        }
    }

    impl Error for SignInError {}
}
