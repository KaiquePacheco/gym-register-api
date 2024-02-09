use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct SignInError {}

impl Display for SignInError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "wrong email or password")
    }
}

impl Error for SignInError {}
