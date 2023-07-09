function handleError(error) {
    console.error(`An error occurred: ${error.message}`);
    const errorDisplay = document.getElementById('errorDisplay');
    errorDisplay.textContent = `An error occurred: ${error.message}`;
    errorDisplay.style.display = 'block';
}

export { handleError };