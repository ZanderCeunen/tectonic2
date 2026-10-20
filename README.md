# Tectonic2

A Rust web application scaffold built for hackathons. Fast, modern, and ready to build on.

## Tech Stack

- **Language**: Rust (edition 2026)
- **Web Framework**: Axum 0.8 (async, type-safe web framework)
- **Database**: SQLite with SQLx (embedded, zero-config database)
- **Runtime**: Tokio (async runtime)
- **Serialization**: Serde & Serde JSON
- **Logging**: Tracing & Tracing Subscriber
- **Error Handling**: Anyhow & Thiserror

## Prerequisites

- Rust 1.83+ (with 2026 edition support)
- Cargo (comes with Rust)

## Quick Start

1. **Clone and navigate to the project**:
   ```bash
   cd tectonic2
   ```

2. **Set up environment variables**:
   ```bash
   cp .env.example .env
   ```

3. **Run the development server**:
   ```bash
   cargo run
   ```

4. **Test the endpoints**:
   ```bash
   curl http://localhost:3000/health
   curl http://localhost:3000/hello
   ```

## Project Structure

```
tectonic2/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Library root and app factory
│   ├── db.rs            # Database connection and migrations
│   ├── handlers.rs      # Request handlers
│   └── routes.rs        # Route definitions
├── migrations/          # SQL migration files
├── .env.example         # Environment variables template
├── Cargo.toml           # Dependencies and project config
└── README.md            # This file
```

## Development Commands

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

### Run with hot reload (requires cargo-watch)
```bash
cargo install cargo-watch
cargo watch -x run
```

### Run tests
```bash
cargo test
```

### Check code without building
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```

## Database

The application uses SQLite with automatic migrations. Database file (`app.db`) is created on first run.

### Running Migrations

Migrations run automatically on startup. To manually run migrations:

```bash
sqlx database create
sqlx migrate run
```

### Schema

The initial schema includes an `items` table:
- `id`: TEXT (UUID, primary key)
- `title`: TEXT (required)
- `description`: TEXT (optional)
- `created_at`: TEXT (timestamp, auto-generated)

## Adding New Features

### Add a new route

1. Add a handler in `src/handlers.rs`:
   ```rust
   pub async fn my_handler() -> Json<Response> {
       Json(Response { /* ... */ })
   }
   ```

2. Register the route in `src/routes.rs`:
   ```rust
   .route("/my-endpoint", get(my_handler))
   ```

### Add a new database table

1. Create a new migration:
   ```bash
   sqlx migrate add create_users_table
   ```

2. Edit the generated SQL file in `migrations/`
3. The migration will run automatically on next startup

## Environment Variables

Copy `.env.example` to `.env` and configure:

- `DATABASE_URL`: SQLite database path (default: `sqlite://app.db`)
- `PORT`: Server port (default: `3000`)
- `RUST_LOG`: Logging level (default: `tectonic2=debug,tower_http=debug`)

## License

MIT
