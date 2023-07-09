1. Exported Variables:
   - `userPreferences` (preferences.js): Stores the user's preferences.
   - `cocktailRecipe` (cocktailGenerator.js): Stores the generated cocktail recipe.

2. Data Schemas:
   - `UserPreferences` (preferences.js): Schema for user's preferences.
   - `CocktailRecipe` (cocktailGenerator.js): Schema for the generated cocktail recipe.

3. ID Names of DOM Elements:
   - `preferencesForm` (index.html): Form where user provides their preferences.
   - `cocktailDisplay` (index.html): Area where the generated cocktail recipe is displayed.

4. Message Names:
   - `PREFERENCES_SUBMITTED` (main.js): Event fired when user submits their preferences.
   - `RECIPE_GENERATED` (main.js): Event fired when a new cocktail recipe is generated.

5. Function Names:
   - `getUserPreferences` (preferences.js): Retrieves user's preferences from the form.
   - `generateCocktail` (cocktailGenerator.js): Generates a cocktail recipe based on user's preferences.
   - `displayCocktail` (main.js): Displays the generated cocktail recipe on the web page.
   - `handleError` (errorHandler.js): Handles any errors or inconsistencies in the app.

6. Shared Dependencies:
   - `main.js` depends on all other files as it orchestrates the overall functionality.
   - `preferences.js` and `cocktailGenerator.js` depend on `cocktailAPI.js` to fetch cocktail data.
   - `errorHandler.js` is used by all other JavaScript files to handle errors.