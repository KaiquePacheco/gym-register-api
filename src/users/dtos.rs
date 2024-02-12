use bcrypt::BcryptError;
use diesel::{pg::Pg, prelude::Insertable, Queryable, Selectable};
use uuid::Uuid;

use super::super::schema::users;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub password_hash: String,
    pub username: String,
    pub email: String,
}

impl<'r> TryFrom<CreateUser<'r>> for User {
    type Error = BcryptError;

    fn try_from(value: CreateUser) -> Result<Self, Self::Error> {
        Ok(Self {
            email: String::from(value.email),
            username: String::from(value.username),
            password_hash: bcrypt::hash(&value.password, value.cost)?,
            id: Uuid::new_v4(),
        })
    }
}

pub struct CreateUser<'r> {
    pub email: &'r str,
    pub password: &'r str,
    pub username: &'r str,
    pub cost: u32,
}
