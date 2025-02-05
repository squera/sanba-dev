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

pub fn list_users(limit: Option<i64>, offset: Option<i64>) -> Result<Vec<User>, ApiError> {
    use domain::schema::user;

    let mut query = user::table.select(user::all_columns).into_boxed();

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(offset) = offset {
        query = query.offset(offset);
    }

    let list = query.load::<User>(&mut establish_connection())?;

    return Ok(list);
}
