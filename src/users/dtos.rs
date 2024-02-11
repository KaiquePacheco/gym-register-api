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
