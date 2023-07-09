```javascript
import { getCocktailData } from './cocktailAPI.js';
import { handleError } from './errorHandler.js';

let cocktailRecipe = {};

const CocktailRecipe = {
    name: '',
    ingredients: [],
    instructions: ''
};

function generateCocktail(userPreferences) {
    getCocktailData(userPreferences)
        .then(data => {
            if (data.drinks && data.drinks.length > 0) {
                const drink = data.drinks[0];
                cocktailRecipe = {
                    ...CocktailRecipe,
                    name: drink.strDrink,
                    ingredients: getIngredients(drink),
                    instructions: drink.strInstructions
                };
                document.dispatchEvent(new CustomEvent('RECIPE_GENERATED'));
            } else {
                throw new Error('No cocktail found for the given preferences');
            }
        })
        .catch(error => {
            handleError(error);
        });
}

function getIngredients(drink) {
    let ingredients = [];
    for (let i = 1; i <= 15; i++) {
        if (drink[`strIngredient${i}`]) {
            ingredients.push({
                ingredient: drink[`strIngredient${i}`],
                measure: drink[`strMeasure${i}`]
            });
        }
    }
    return ingredients;
}

export { cocktailRecipe, generateCocktail };
```