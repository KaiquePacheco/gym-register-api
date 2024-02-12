use diesel::{insert_into, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use std::error::Error;
use std::sync::Arc;

use super::{
    super::schema::users,
    dtos::{CreateUser, User},
};

#[derive(Clone)]
pub struct UsersRepository(pub Arc<Pool<AsyncPgConnection>>);

impl UsersRepository {
    pub async fn find_by_email(&self, email: &str) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = self.0.get().await?;

        Ok(users::table
            .select(User::as_select())
            .filter(users::email.eq(email))
            .get_results(&mut conn)
            .await?)
    }

    pub async fn create<'r>(&self, create_user: CreateUser<'r>) -> Result<User, Box<dyn Error>> {
        let mut conn = self.0.get().await?;

        let user = User::try_from(create_user)?;

        insert_into(users::table)
            .values(&user)
            .execute(&mut conn)
            .await?;

        Ok(user)
    }
}
