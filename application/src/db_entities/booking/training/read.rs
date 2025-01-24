use diesel::prelude::*;
use domain::models::{
    full_tables::{TrainingPlayer, TrainingPlayerTag},
    others::TrainingPlayerWithTags,
};
use infrastructure::establish_connection;
use shared::response_models::ApiError;

pub fn get_training_player_list(training_id: i64) -> Result<Vec<TrainingPlayerWithTags>, ApiError> {
    use domain::schema::{training_player, training_player_tag};

    let connection = &mut establish_connection();

    let mut training_players_with_tags: Vec<TrainingPlayerWithTags> = training_player::table
        .filter(training_player::training_id.eq(&training_id))
        .select(TrainingPlayer::as_select())
        .load(connection)?
        .into_iter()
        .map(|training_player| TrainingPlayerWithTags {
            id: training_player.id,
            training_id: training_player.training_id,
            player_id: training_player.player_id,
            rfid_tag_ids: vec![],
        })
        .collect();

    let tags = training_player_tag::table
        .filter(training_player_tag::training_id.eq(&training_id))
        .load::<TrainingPlayerTag>(connection)?;

    populate_rfid_tags(&mut training_players_with_tags, &tags);

    return Ok(training_players_with_tags);
}

fn populate_rfid_tags(
    training_players: &mut Vec<TrainingPlayerWithTags>,
    training_player_tags: &[TrainingPlayerTag],
) {
    // Creiamo una mappa da player_id a un vettore di rfid_tag_id per accesso rapido
    let mut tags_map: std::collections::HashMap<i64, Vec<i64>> = std::collections::HashMap::new();

    for tag in training_player_tags {
        tags_map
            .entry(tag.player_id)
            .or_default()
            .push(tag.rfid_tag_id);
    }

    // Aggiorniamo il campo rfid_tag_ids per ogni TrainingPlayerWithTags
    for player in training_players {
        if let Some(tag_ids) = tags_map.get(&player.player_id) {
            player.rfid_tag_ids = tag_ids.clone();
        } else {
            player.rfid_tag_ids = Vec::new(); // Nessun tag associato
        }
    }
}
