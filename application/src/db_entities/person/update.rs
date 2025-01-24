use diesel::prelude::*;
use diesel::result::Error;
use domain::models::{
    full_tables::{Administrator, Coach, CoachTeam, Fan, Person, Player, PlayerTeam},
    insertions::{NewCoachTeam, NewPlayerTeam},
    others::{JoinInfo, LeaveInfo, NewProfile, PersonWithUser},
};
use infrastructure::establish_connection;
use rocket::http::Status;
use shared::response_models::{ApiError, ApiErrorType};

use crate::{
    authentication::Claims,
    authorization::{
        person_checks::{is_administrator, is_person_with_user},
        team_checks::{is_coach_of_team, is_responsible_of_team},
        user_checks::is_same_user,
    },
    db_entities::person::read::find_person,
};

pub fn authorize_update_person(
    requesting_user: Claims,
    new_person: Person,
) -> Result<Person, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        if is_person_with_user(new_person.id)? {
            if is_same_user(requesting_user.subject_id, new_person.id) {
                is_authorized = true;
            }
        } else {
            is_authorized = true;
        }
    }

    if is_authorized {
        return update_person(new_person);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to update person {}",
                requesting_user.subject_id, new_person.id
            ),
        });
    }
}

pub(crate) fn update_person(new_person: Person) -> Result<Person, ApiError> {
    let connection = &mut establish_connection();

    let updated_person: Person = match connection.transaction::<_, Error, _>(|connection| {
        // diesel::update(person)
        //     .filter(id.eq(new_person.id))
        //     .set(&new_person)
        //     .execute(connection)?;

        // person.find(new_person.id).first(connection)

        new_person.save_changes(connection)
    }) {
        Ok(p) => p,
        Err(err) => {
            return Err(ApiError {
                http_status: Status::InternalServerError,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: format!("Error while updating person - {}", err),
            })
        }
    };

    return Ok(updated_person);
}

pub fn authorize_add_profile(
    requesting_user: Claims,
    person_id: i64,
    new_profile: NewProfile,
) -> Result<PersonWithUser, ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        if is_person_with_user(person_id)? {
            if is_same_user(requesting_user.subject_id, person_id) {
                is_authorized = true;
            }
        } else {
            is_authorized = true;
        }
    }

    if is_authorized {
        return add_profile(person_id, new_profile);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to add profile to person {}",
                requesting_user.subject_id, person_id
            ),
        });
    }
}

pub(crate) fn add_profile(
    person_id: i64,
    new_profile: NewProfile,
) -> Result<PersonWithUser, ApiError> {
    use domain::schema::{administrator, coach, fan, player};

    let connection = &mut establish_connection();

    if new_profile.administrator.is_some() && new_profile.administrator.unwrap() {
        diesel::insert_into(administrator::table)
            .values(&Administrator {
                person_id: person_id,
            })
            .execute(connection)?;
    }

    if new_profile.coach.is_some() {
        let new_coach = new_profile.coach.unwrap();
        diesel::insert_into(coach::table)
            .values(&Coach {
                person_id: person_id,
                role: new_coach.role,
            })
            .execute(connection)?;
    }

    if new_profile.fan.is_some() && new_profile.fan.unwrap() {
        diesel::insert_into(fan::table)
            .values(&Fan {
                person_id: person_id,
            })
            .execute(connection)?;
    }

    if new_profile.player.is_some() && new_profile.player.unwrap() {
        diesel::insert_into(player::table)
            .values(&Player {
                person_id: person_id,
            })
            .execute(connection)?;
    }

    let person = find_person(person_id)?;
    return Ok(person);
}

pub fn authorize_join_team(
    requesting_user: Claims,
    person_id: i64,
    team_id: i64,
    join_info: JoinInfo,
) -> Result<(), ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        if is_coach_of_team(requesting_user.subject_id, Some(team_id), true)?
            || is_responsible_of_team(requesting_user.subject_id, team_id, true)?
        {
            is_authorized = true;
        }
    }

    if is_authorized {
        return join_team(person_id, team_id, join_info);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to let user {} join team {}",
                requesting_user.subject_id, person_id, team_id
            ),
        });
    }
}

pub(crate) fn join_team(person_id: i64, team_id: i64, join_info: JoinInfo) -> Result<(), ApiError> {
    use domain::schema::{coach_team, player_team};

    let connection = &mut establish_connection();

    match join_info.role {
        0 => {
            diesel::insert_into(player_team::table)
                .values(&NewPlayerTeam {
                    player_id: person_id,
                    team_id: team_id,
                    since_date: join_info.since_date,
                    until_date: join_info.until_date,
                })
                .execute(connection)?;
        }
        1 => {
            diesel::insert_into(coach_team::table)
                .values(&NewCoachTeam {
                    coach_id: person_id,
                    team_id: team_id,
                    since_date: join_info.since_date,
                    until_date: join_info.until_date,
                })
                .execute(connection)?;
        }
        _ => {
            return Err(ApiError {
                http_status: Status::BadRequest,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: "Invalid role".to_string(),
            })
        }
    }

    return Ok(());
}

pub fn authorize_leave_team(
    requesting_user: Claims,
    person_id: i64,
    team_id: i64,
    leave_info: LeaveInfo,
) -> Result<(), ApiError> {
    let mut is_authorized = false;
    if is_administrator(requesting_user.subject_id)? {
        is_authorized = true;
    } else {
        if is_person_with_user(person_id)? && is_same_user(requesting_user.subject_id, person_id) {
            is_authorized = true;
        } else {
            if is_coach_of_team(requesting_user.subject_id, Some(team_id), true)?
                || is_responsible_of_team(requesting_user.subject_id, team_id, true)?
            {
                is_authorized = true;
            }
        }
    }

    if is_authorized {
        return leave_team(person_id, team_id, leave_info);
    } else {
        return Err(ApiError {
            http_status: Status::Forbidden,
            error_code: 123, // TODO organizzare i codici di errore
            error_type: ApiErrorType::AuthorizationError,
            message: format!(
                "Error - User {} is not authorized to let user {} leave team {}",
                requesting_user.subject_id, person_id, team_id
            ),
        });
    }
}

pub(crate) fn leave_team(
    person_id: i64,
    team_id: i64,
    leave_info: LeaveInfo,
) -> Result<(), ApiError> {
    use domain::schema::{coach_team, player_team};

    let connection = &mut establish_connection();

    match leave_info.role {
        0 => {
            match connection.transaction::<_, Error, _>(|connection| {
                let mut p_t: PlayerTeam = player_team::table
                    .filter(player_team::player_id.eq(person_id))
                    .filter(player_team::team_id.eq(team_id))
                    .select(PlayerTeam::as_select())
                    .get_result(connection)?;

                p_t.until_date = Some(chrono::Utc::now().naive_utc());

                p_t.save_changes::<PlayerTeam>(connection)
            }) {
                Ok(_) => return Ok(()),
                Err(err) => {
                    return Err(ApiError {
                        http_status: Status::InternalServerError,
                        error_code: 123,
                        error_type: ApiErrorType::ApplicationError,
                        message: format!(
                            "Error while updating relation between team and player - {}",
                            err
                        ),
                    })
                }
            };
        }
        1 => {
            match connection.transaction::<_, Error, _>(|connection| {
                let mut p_t: CoachTeam = coach_team::table
                    .filter(coach_team::coach_id.eq(person_id))
                    .filter(coach_team::team_id.eq(team_id))
                    .select(CoachTeam::as_select())
                    .get_result(connection)?;

                p_t.until_date = Some(chrono::Utc::now().naive_utc());

                p_t.save_changes::<CoachTeam>(connection)
            }) {
                Ok(_) => return Ok(()),
                Err(err) => {
                    return Err(ApiError {
                        http_status: Status::InternalServerError,
                        error_code: 123,
                        error_type: ApiErrorType::ApplicationError,
                        message: format!(
                            "Error while updating relation between team and coach - {}",
                            err
                        ),
                    })
                }
            };
        }
        _ => {
            return Err(ApiError {
                http_status: Status::BadRequest,
                error_code: 123,
                error_type: ApiErrorType::ApplicationError,
                message: "Invalid role".to_string(),
            })
        }
    }
}
