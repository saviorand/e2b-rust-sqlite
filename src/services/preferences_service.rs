use crate::models::preferences::Preferences;
use crate::utils::db_connector::connect_to_db;

pub struct PreferencesService;

impl PreferencesService {
    pub fn get_preferences(user_id: i32) -> Result<Preferences, &'static str> {
        let connection = connect_to_db();
        let preferences = Preferences::find(user_id, &connection);

        match preferences {
            Ok(preferences) => Ok(preferences),
            Err(_) => Err("Error fetching preferences"),
        }
    }

    pub fn update_preferences(user_id: i32, new_preferences: Preferences) -> Result<(), &'static str> {
        let connection = connect_to_db();
        let result = Preferences::update(user_id, new_preferences, &connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error updating preferences"),
        }
    }
}