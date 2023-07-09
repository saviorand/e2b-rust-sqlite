```javascript
document.addEventListener('DOMContentLoaded', function() {
    const preferencesForm = document.getElementById('preferencesForm');
    const cocktailDisplay = document.getElementById('cocktailDisplay');

    preferencesForm.addEventListener('submit', function(event) {
        event.preventDefault();
        try {
            const userPreferences = getUserPreferences();
            document.dispatchEvent(new CustomEvent('PREFERENCES_SUBMITTED', { detail: userPreferences }));
        } catch (error) {
            handleError(error);
        }
    });

    document.addEventListener('PREFERENCES_SUBMITTED', async function(event) {
        try {
            const cocktailRecipe = await generateCocktail(event.detail);
            document.dispatchEvent(new CustomEvent('RECIPE_GENERATED', { detail: cocktailRecipe }));
        } catch (error) {
            handleError(error);
        }
    });

    document.addEventListener('RECIPE_GENERATED', function(event) {
        try {
            displayCocktail(event.detail, cocktailDisplay);
        } catch (error) {
            handleError(error);
        }
    });
});
```