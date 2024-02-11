use std::env;

use dotenvy::dotenv_override;
use hmac::{digest::KeyInit, Hmac};
use sha2::Sha256;

pub struct Configs {
    pub jwt_key: Hmac<Sha256>,

    pub db_url: String,
    pub db_max_conns: usize,

    pub bcrypt_cost: u32,
}

impl Default for Configs {
    fn default() -> Self {
        let _ = dotenv_override();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");
        let db_max_conns = env::var("DATABASE_MAX_CONNECTIONS")
            .expect("DATABASE_MAX_CONNECTIONS environment variable not found")
            .parse()
            .expect("Could not parse to unsigned integer the environment variable 'DATABASE_MAX_CONNECTIONS'");

        let jwt_secret =
            env::var("JWT_SECRET").expect("DATABASE_URL environment variable not found");
        let jwt_key = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

        let bcrypt_cost = env::var("BCRYPT_COST")
            .expect("BCRYPT_COST environment variable not found")
            .parse()
            .expect("Could not parse to unsigned integer the environment variable 'DATABASE_MAX_CONNECTIONS'");

        Self {
            jwt_key,
            db_url,
            db_max_conns,
            bcrypt_cost,
        }
    }
}
