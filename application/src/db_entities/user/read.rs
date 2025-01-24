use diesel::prelude::*;
use domain::models::full_tables::User;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn find_user(user_id: i64) -> Result<User, ApiError> {
    use domain::schema::user;

    let user = user::table
        .find(user_id)
        .first::<User>(&mut establish_connection())?;

    return Ok(user);
}

pub fn list_users() -> Result<Vec<User>, ApiError> {
    use domain::schema::user;

    let list = user::table
        .select(user::all_columns)
        .load::<User>(&mut establish_connection())?;

    return Ok(list);
}
