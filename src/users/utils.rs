use diesel::{result::Error, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Object, AsyncPgConnection, RunQueryDsl};

use super::dtos::UserData;

pub async fn find_by_email(
    mut conn: Object<AsyncPgConnection>,
    email: &str,
) -> Result<Vec<UserData>, Error> {
    use super::super::schema::user;

    user::table
        .select(UserData::as_select())
        .filter(user::email.eq(email))
        .get_results(&mut conn)
        .await
}
