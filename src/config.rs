use std::env;

use dotenvy::dotenv_override;

pub struct Configs {
    pub db_url: String,
    pub db_max_conns: usize,
}

impl Default for Configs {
    fn default() -> Self {
        let _ = dotenv_override();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

        let db_max_conns = env::var("DATABASE_MAX_CONNECTIONS")
            .expect("DATABASE_MAX_CONNECTIONS environment variable not found")
            .parse()
            .expect("Could not parse to unsigned integer the environment variable 'DATABASE_MAX_CONNECTIONS'");

        Self {
            db_url,
            db_max_conns,
        }
    }
}
