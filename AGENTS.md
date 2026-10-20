# Project Rules and Commands

## Verification Commands

Before making changes or after completing features, don't run any of these commands: 

### Quick Check
```bash
cargo check
```

### Format Code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

### Run Tests
```bash
cargo test
```

### Full Build
```bash
cargo build
```
## The user will do these on their own! 


## Development Workflow

1. Make changes to the codebase
2. The user will run the verification commands

## Database Management

- Migrations run automatically on startup
- Database file: `app.db` (in project root)
- Migrations location: `migrations/`
- To add a new migration: `sqlx migrate add <migration_name>`

## Code Style

- Use existing patterns in the codebase
- Follow Rust idiomatic conventions
- Keep functions focused and small
- Use Result<T> for error handling with anyhow::Error
- Use serde for all JSON serialization

## Key Files

- `src/main.rs` - Application entry point
- `src/lib.rs` - Library root
- `src/db.rs` - Database setup
- `src/handlers.rs` - API handlers
- `src/routes.rs` - Route definitions
- `Cargo.toml` - Dependencies
- `migrations/` - SQL migrations

## Work mapping agent to agent
Edit this file as you are working to inform the other agents working on this. 