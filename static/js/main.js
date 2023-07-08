document.addEventListener('DOMContentLoaded', (event) => {
    const preferencesForm = document.getElementById('user-preferences');
    const recipeDisplay = document.getElementById('recipe-result');

    preferencesForm.addEventListener('submit', (event) => {
        event.preventDefault();

        const formData = new FormData(preferencesForm);
        const preferences = Object.fromEntries(formData);

        fetch('/preferences', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(preferences)
        })
        .then(response => response.json())
        .then(data => {
            if (data.message === 'PREFERENCES_SUBMITTED') {
                getRecipe();
            }
        })
        .catch(error => console.error('Error:', error));
    });

    function getRecipe() {
        fetch('/recipe')
        .then(response => response.json())
        .then(data => {
            if (data.message === 'RECIPE_GENERATED') {
                recipeDisplay.textContent = data.recipe;
            }
        })
        .catch(error => console.error('Error:', error));
    }
});