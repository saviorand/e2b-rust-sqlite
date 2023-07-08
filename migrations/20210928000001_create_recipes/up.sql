CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    ingredients TEXT NOT NULL,
    instructions TEXT NOT NULL,
    preference_id INTEGER NOT NULL REFERENCES preferences(id)
);