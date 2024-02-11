use diesel::{insert_into, result::Error, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use std::error::Error as ErrorTrait;

use super::super::schema::users;

use super::dtos::User;

pub async fn find_by_email(
    mut conn: Object<AsyncPgConnection>,

    email: &str,
) -> Result<Vec<User>, Error> {
    users::table
        .select(User::as_select())
        .filter(users::email.eq(email))
        .get_results(&mut conn)
        .await
}

pub async fn create(
    mut conn: Object<AsyncPgConnection>,

    username: String,
    email: String,
    password: String,
    bcrypt_cost: u32,
) -> Result<User, Box<dyn ErrorTrait>> {
    let user = User {
        email,
        username,
        password_hash: bcrypt::hash(password, bcrypt_cost)?,
        id: Uuid::new_v4(),
    };

    insert_into(users::table)
        .values(&user)
        .execute(&mut conn)
        .await?;

    Ok(user)
}
