# Milkshake Recipe Generator

This is a full-stack application built with Rust. The user can provide their preferences and get a personalized recipe of a milkshake.

## Prerequisites

- Rust 1.54.0 or later
- Diesel CLI for database migrations
- PostgreSQL

## Setup

1. Clone the repository:

```bash
git clone https://github.com/username/milkshake-recipe-generator.git
cd milkshake-recipe-generator
```

2. Install Diesel CLI with PostgreSQL features:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

3. Setup the database:

```bash
echo DATABASE_URL=postgres://username:password@localhost/milkshake_recipe_generator > .env
diesel setup
```

4. Run the migrations:

```bash
diesel migration run
```

## Running the App

1. Build the application:

```bash
cargo build
```

2. Run the application:

```bash
cargo run
```

The application will start on `localhost:8000`.

## Testing

To run the tests:

```bash
cargo test
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)