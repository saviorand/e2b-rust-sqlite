# Milkshake Recipe Generator

This is a full-stack application built with Rust. The user can provide their preferences and get a personalized recipe of a milkshake.

## Prerequisites

- Rust 1.54.0 or later
- Diesel CLI for database migrations

## Setup

1. Clone the repository:

```bash
git clone https://github.com/user/milkshake-recipe-generator.git
cd milkshake-recipe-generator
```

2. Install Diesel CLI with PostgreSQL features:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

3. Setup the database:

```bash
diesel setup
```

4. Run database migrations:

```bash
diesel migration run
```

## Running the Application

1. Build the application:

```bash
cargo build
```

2. Run the application:

```bash
cargo run
```

The application will start and listen on `localhost:8000`.

## Testing the Application

1. Build the tests:

```bash
cargo test
```

2. Run the tests:

```bash
cargo test -- --test-threads=1
```

## API Endpoints

- `GET /`: Home page
- `POST /preferences`: Submit user preferences
- `GET /recipe`: Get personalized milkshake recipe based on user preferences

## License

This project is licensed under the MIT License.