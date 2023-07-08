# Milkshake Recipe Generator

This is a full-stack application built with Rust. Users can provide their preferences and get a personalized recipe of a milkshake.

## Prerequisites

- Rust 1.54.0 or later
- Cargo 1.54.0 or later

## Installation

Clone the repository:

```
git clone https://github.com/user/milkshake-recipe-generator.git
cd milkshake-recipe-generator
```

Build the project:

```
cargo build
```

## Running the Application

To run the application:

```
cargo run
```

The application will start and listen on `http://localhost:8000`.

## Usage

- To set your preferences, make a POST request to `http://localhost:8000/preferences` with your preferences in the request body.
- To get a personalized milkshake recipe based on your preferences, make a GET request to `http://localhost:8000/recipe`.

## Testing

To run the tests:

```
cargo test
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)