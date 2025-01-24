use diesel::prelude::*;
use domain::models::full_tables::Camera;
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn find_camera(camera_id: i64) -> Result<Camera, ApiError> {
    use domain::schema::camera;

    let connection = &mut establish_connection();

    let camera = camera::table
        .filter(camera::id.eq(camera_id))
        .select(Camera::as_select())
        .get_result(connection)?;

    return Ok(camera);
}

pub fn list_cameras() -> Result<Vec<Camera>, ApiError> {
    use domain::schema::camera;

    let connection = &mut establish_connection();

    let cameras = camera::table.select(Camera::as_select()).load(connection)?;

    return Ok(cameras);
}
