use std::collections::HashMap;

use diesel::prelude::*;
use domain::models::{
    full_tables::{Administrator, Coach, CoachTeam, Fan, Person, Player, PlayerTeam, Team, User},
    others::{CoachProfile, PersonWithUser, PlayerProfile, ProfileSet, TeamRelation, TeamStaff},
};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn find_person(person_id: i64) -> Result<PersonWithUser, ApiError> {
    use domain::schema::person;

    let connection = &mut establish_connection();

    let p = person::table
        .filter(person::id.eq(person_id))
        .select(Person::as_select())
        .get_result(connection)?;

    let u = User::belonging_to(&p)
        .select(User::as_select())
        .load(connection)?;

    return Ok(PersonWithUser {
        person: p,
        user: u.into_iter().next(),
    });
}

pub fn list_people() -> Result<Vec<PersonWithUser>, ApiError> {
    use domain::schema::person;

    let connection = &mut establish_connection();

    let all_people = person::table.select(Person::as_select()).load(connection)?;

    // get all users for all people
    let users = User::belonging_to(&all_people)
        .select(User::as_select())
        .load(connection)?;

    // group the user per person
    let people_with_users = users
        .grouped_by(&all_people)
        .into_iter()
        .zip(all_people)
        .map(|(user, person)| PersonWithUser {
            person: person,
            user: user.into_iter().next(),
        })
        .collect::<Vec<PersonWithUser>>();

    return Ok(people_with_users);
}

pub fn get_profiles(person_id: i64) -> Result<ProfileSet, ApiError> {
    let _ = find_person(person_id)?; // check if person exists

    return Ok(ProfileSet {
        administrator: if get_admin_profile(person_id)?.is_some() {
            Some(true)
        } else {
            None
        },
        coach: get_coach_profile(person_id)?,
        fan: if get_fan_profile(person_id)?.is_some() {
            Some(true)
        } else {
            None
        },
        player: get_player_profile(person_id)?,
    });
}

fn get_admin_profile(admin_id: i64) -> Result<Option<()>, ApiError> {
    use domain::schema::administrator;

    let connection = &mut establish_connection();

    let admin_data = administrator::table
        .filter(administrator::person_id.eq(admin_id))
        .select(Administrator::as_select())
        .first(connection)
        .optional()?;

    match admin_data {
        Some(_) => return Ok(Some(())),
        None => return Ok(None),
    }
}

fn get_coach_profile(coach_id: i64) -> Result<Option<CoachProfile>, ApiError> {
    use domain::schema::{coach, coach_team, team};

    let connection = &mut establish_connection();

    let coach_data = coach::table
        .filter(coach::person_id.eq(coach_id))
        .select(Coach::as_select())
        .first(connection)
        .optional()?;

    match coach_data {
        Some(coach_data) => {
            let coach_team_team: Vec<(Team, CoachTeam)> = team::table
                .inner_join(coach_team::table)
                .filter(coach_team::coach_id.eq(coach_id))
                .select((Team::as_select(), CoachTeam::as_select()))
                .load::<(Team, CoachTeam)>(connection)?;

            let coach_relations: Vec<TeamRelation> = coach_team_team
                .into_iter()
                .map(|(t, ct)| TeamRelation {
                    team: t,
                    since_date: ct.since_date,
                    until_date: ct.until_date,
                })
                .collect();

            return Ok(Some(CoachProfile {
                role: coach_data.role,
                profiles: coach_relations,
            }));
        }
        None => return Ok(None),
    }
}

fn get_fan_profile(fan_id: i64) -> Result<Option<()>, ApiError> {
    use domain::schema::fan;

    let connection = &mut establish_connection();

    let fan_data = fan::table
        .filter(fan::person_id.eq(fan_id))
        .select(Fan::as_select())
        .first(connection)
        .optional()?;

    match fan_data {
        Some(_) => return Ok(Some(())),
        None => return Ok(None),
    }
}

fn get_player_profile(player_id: i64) -> Result<Option<PlayerProfile>, ApiError> {
    use domain::schema::{player, player_team, team};

    let connection = &mut establish_connection();

    let player_data = player::table
        .filter(player::person_id.eq(player_id))
        .select(Player::as_select())
        .first(connection)
        .optional()?;

    match player_data {
        Some(_) => {
            let player_team_team: Vec<(Team, PlayerTeam)> = team::table
                .inner_join(player_team::table)
                .filter(player_team::player_id.eq(player_id))
                .select((Team::as_select(), PlayerTeam::as_select()))
                .load::<(Team, PlayerTeam)>(connection)?;

            let player_relations: Vec<TeamRelation> = player_team_team
                .into_iter()
                .map(|(t, pt)| TeamRelation {
                    team: t,
                    since_date: pt.since_date,
                    until_date: pt.until_date,
                })
                .collect();

            return Ok(Some(PlayerProfile {
                profiles: player_relations,
            }));
        }
        None => return Ok(None),
    }
}

// struct temporanea per tenere solo gli id dei giocatori e degli allenatori di un team
struct TeamStaffIds {
    team_id: i64,
    player_ids: Vec<i64>,
    coach_ids: Vec<i64>,
}

impl TeamStaffIds {
    // risolve gli id in oggetti completi
    fn to_team_staff(&self) -> TeamStaff {
        let players_with_users = get_people_from_ids(self.player_ids.clone()).unwrap();
        let coaches_with_users = get_people_from_ids(self.coach_ids.clone()).unwrap();

        TeamStaff {
            team_id: self.team_id,
            players: players_with_users,
            coaches: coaches_with_users,
        }
    }
}

pub fn list_people_by_teams(team_ids: Vec<i64>) -> Result<Vec<TeamStaff>, ApiError> {
    let players_per_team = list_players_by_teams(&team_ids)?;
    let coaches_per_team = list_coaches_by_teams(&team_ids)?;

    let team_staff_ids = combine_team_data(players_per_team, coaches_per_team);

    let team_staff = team_staff_ids
        .into_iter()
        .map(|team_staff_id| team_staff_id.to_team_staff())
        .collect();

    return Ok(team_staff);
}

fn list_players_by_teams(team_ids: &Vec<i64>) -> Result<Vec<(Team, Vec<i64>)>, ApiError> {
    use domain::schema::{player, team};

    let connection = &mut establish_connection();

    let teams = team::table
        .filter(team::id.eq_any(team_ids))
        .select(Team::as_select())
        .load::<Team>(connection)?;

    let players = PlayerTeam::belonging_to(&teams)
        .inner_join(player::table)
        .select((PlayerTeam::as_select(), Player::as_select()))
        .load(connection)?;

    let players_per_team: Vec<(Team, Vec<i64>)> = players
        .grouped_by(&teams)
        .into_iter()
        .zip(teams)
        .map(|(p, team)| {
            (
                team,
                p.into_iter().map(|(_, player)| player.person_id).collect(),
            )
        })
        .collect();

    return Ok(players_per_team);
}

fn list_coaches_by_teams(team_ids: &Vec<i64>) -> Result<Vec<(Team, Vec<i64>)>, ApiError> {
    use domain::schema::{coach, team};

    let connection = &mut establish_connection();

    let teams = team::table
        .filter(team::id.eq_any(team_ids))
        .select(Team::as_select())
        .load::<Team>(connection)?;

    let coaches = CoachTeam::belonging_to(&teams)
        .inner_join(coach::table)
        .select((CoachTeam::as_select(), Coach::as_select()))
        .load(connection)?;

    let coaches_per_team: Vec<(Team, Vec<i64>)> = coaches
        .grouped_by(&teams)
        .into_iter()
        .zip(teams)
        .map(|(c, team)| {
            (
                team,
                c.into_iter().map(|(_, coach)| coach.person_id).collect(),
            )
        })
        .collect();

    return Ok(coaches_per_team);
}

pub fn list_people_by_club(club_id: String) -> Result<Vec<TeamStaff>, ApiError> {
    use domain::schema::team;

    let connection = &mut establish_connection();

    let team_ids = team::table
        .filter(team::club_id.eq(club_id))
        .select(team::id)
        .load(connection)?;

    let res = list_people_by_teams(team_ids)?;

    return Ok(res);
}

// Funzione per ottenere le informazioni complete da una lista di id persone
fn get_people_from_ids(person_ids: Vec<i64>) -> Result<Vec<PersonWithUser>, ApiError> {
    use domain::schema::person;

    let connection = &mut establish_connection();

    let people = person::table
        .filter(person::id.eq_any(person_ids))
        .select(Person::as_select())
        .load::<Person>(connection)?;

    let users = User::belonging_to(&people)
        .select(User::as_select())
        .load(connection)?;

    let people_with_users = users
        .grouped_by(&people)
        .into_iter()
        .zip(people)
        .map(|(user, person)| PersonWithUser {
            person: person,
            user: user.into_iter().next(),
        })
        .collect::<Vec<PersonWithUser>>();

    Ok(people_with_users)
}

fn combine_team_data(
    players: Vec<(Team, Vec<i64>)>,
    coaches: Vec<(Team, Vec<i64>)>,
) -> Vec<TeamStaffIds> {
    // Creare una HashMap per un rapido accesso agli allenatori in base al team_id
    let mut coach_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for (team, coach_ids) in coaches {
        coach_map.insert(team.id, coach_ids);
    }

    // Costruire il risultato unendo le informazioni di giocatori e allenatori
    let mut team_staffs = Vec::new();
    for (team, player_ids) in players {
        let coach_ids = coach_map.remove(&team.id).unwrap_or_else(Vec::new); // Se non ci sono allenatori, usa un vettore vuoto
        team_staffs.push(TeamStaffIds {
            team_id: team.id,
            player_ids: player_ids,
            coach_ids: coach_ids,
        });
    }

    team_staffs
}
