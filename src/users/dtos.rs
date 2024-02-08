use diesel::{pg::Pg, Queryable, Selectable};
use uuid::Uuid;

use super::super::schema::user;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(Pg))]
pub struct UserData {
    pub id: Uuid,
    pub password_hash: String,
    pub username: String,
    pub email: String,
}
