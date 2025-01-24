use diesel::prelude::*;
use domain::models::{
    full_tables::{FormationPlayer, FormationPlayerTag},
    others::FormationPlayerWithTags,
};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn get_formation_player_list(
    formation_id: i64,
) -> Result<Vec<FormationPlayerWithTags>, ApiError> {
    use domain::schema::{formation_player, formation_player_tag};

    let connection = &mut establish_connection();

    let mut formation_players_with_tags: Vec<FormationPlayerWithTags> = formation_player::table
        .filter(formation_player::formation_id.eq(&formation_id))
        .select(FormationPlayer::as_select())
        .load(connection)?
        .into_iter()
        .map(|formation_player| FormationPlayerWithTags {
            id: formation_player.id,
            formation_id: formation_player.formation_id,
            player_id: formation_player.player_id,
            rfid_tag_ids: vec![],
        })
        .collect();

    let tags = formation_player_tag::table
        .filter(formation_player_tag::formation_id.eq(&formation_id))
        .load::<FormationPlayerTag>(connection)?;

    populate_rfid_tags(&mut formation_players_with_tags, &tags);

    return Ok(formation_players_with_tags);
}

fn populate_rfid_tags(
    formation_players: &mut Vec<FormationPlayerWithTags>,
    formation_player_tags: &[FormationPlayerTag],
) {
    // Creiamo una mappa da player_id a un vettore di rfid_tag_id per accesso rapido
    let mut tags_map: std::collections::HashMap<i64, Vec<i64>> = std::collections::HashMap::new();

    for tag in formation_player_tags {
        tags_map
            .entry(tag.player_id)
            .or_default()
            .push(tag.rfid_tag_id);
    }

    // Aggiorniamo il campo rfid_tag_ids per ogni FormationPlayerWithTags
    for player in formation_players {
        if let Some(tag_ids) = tags_map.get(&player.player_id) {
            player.rfid_tag_ids = tag_ids.clone();
        } else {
            player.rfid_tag_ids = Vec::new(); // Nessun tag associato
        }
    }
}
