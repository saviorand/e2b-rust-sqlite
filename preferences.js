```javascript
let userPreferences = {};

const UserPreferences = {
    alcohol: "",
    sweetness: "",
    fruitiness: "",
    strength: ""
};

function getUserPreferences() {
    const form = document.getElementById('preferencesForm');
    userPreferences.alcohol = form.elements['alcohol'].value;
    userPreferences.sweetness = form.elements['sweetness'].value;
    userPreferences.fruitiness = form.elements['fruitiness'].value;
    userPreferences.strength = form.elements['strength'].value;
    return userPreferences;
}

document.getElementById('preferencesForm').addEventListener('submit', function(event) {
    event.preventDefault();
    getUserPreferences();
    document.dispatchEvent(new CustomEvent('PREFERENCES_SUBMITTED', { detail: userPreferences }));
});
```