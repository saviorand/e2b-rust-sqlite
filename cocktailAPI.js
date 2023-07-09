const fetch = require('node-fetch');

async function getCocktailData(userPreferences) {
    try {
        const response = await fetch(`https://www.thecocktaildb.com/api/json/v1/1/filter.php?i=${userPreferences}`);
        const data = await response.json();
        return data.drinks;
    } catch (error) {
        handleError(error);
    }
}

module.exports = getCocktailData;