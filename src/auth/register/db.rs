use diesel::{RunQueryDsl, SelectableHelper};

use crate::users::{
    db::{DbConnection, DbResult},
    model::{NewUser, User},
    schema::users,
};

pub fn register_user(conn: &mut DbConnection, new_user: NewUser) -> DbResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}
