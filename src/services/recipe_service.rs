```rust
use crate::models::recipe::Recipe;
use crate::models::preferences::Preferences;
use crate::utils::db_connector::establish_connection;

pub struct RecipeService;

impl RecipeService {
    pub fn get_recipe_based_on_preferences(preferences: Preferences) -> Recipe {
        let connection = establish_connection();
        let recipe_query = format!(
            "SELECT * FROM recipes WHERE flavor = '{}' AND sweetness = '{}' AND creaminess = '{}'",
            preferences.flavor, preferences.sweetness, preferences.creaminess
        );

        let recipe: Recipe = diesel::sql_query(recipe_query)
            .get_result(&connection)
            .expect("Error loading recipe");

        recipe
    }
}
```