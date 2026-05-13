# Library System — Microservice Sample

A reference Rust workspace demonstrating microservice patterns. Three independent services manage users, books, and reviews for a library system. Each service owns its database, follows Domain-Driven Design layering, and is independently deployable.

This project is intentionally incremental — authentication, inter-service communication, and deployment will be added in later phases.

## Services

| Service | Port | Responsibility |
|---------|------|----------------|
| `user-service` | 3001 | Create and look up library members |
| `book-service` | 3002 | Manage the books catalog |
| `review-service` | 3003 | Submit and read book reviews |

## Tech stack

- **Axum** — async HTTP framework
- **Diesel** — compile-time checked SQL ORM
- **PostgreSQL 16** — one database per service
- **deadpool-diesel** — async connection pool
- **Embedded migrations** — applied automatically on startup
- **utoipa + utoipa-axum** — OpenAPI 3 spec auto-generated from handler annotations, served at `/api-docs/openapi.json`

## Prerequisites

- [Nix](https://nixos.org/download) with flakes enabled — provides all other tools
- [direnv](https://direnv.net) — auto-loads the dev shell on `cd` into the project
- Podman (for running containers)

Without Nix, you'll need Rust stable, podman-compose, and `diesel_cli` (`cargo install diesel_cli --no-default-features --features postgres`) installed manually.

## Quickstart

```bash
# 1. Allow direnv to activate the dev shell automatically
direnv allow
# The shell is now loaded whenever you enter this directory.
# All tools (cargo, diesel_cli, podman-compose, cargo-watch) are available.

# 2. Start all three Postgres databases
podman compose up -d

# 3. Configure environment variables
cp .env.example .env
# Edit .env if your local setup differs from the defaults

# 4. Start a service (migrations run automatically on startup)
cargo run -p user-service
cargo run -p book-service
cargo run -p review-service
```

## Building OCI images

Images are built reproducibly with Nix and crane — no Dockerfile required.

```bash
# Build a service image (produces ./result, a tarball)
nix build .#user-service-image
nix build .#book-service-image
nix build .#review-service-image

# Load and run
podman load < result
podman run --env-file .env user-service
```

Images use a distroless base — no shell, minimal attack surface.

## API Reference

### user-service — `localhost:3001`

```bash
# Create a user
curl -s -X POST localhost:3001/users \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","display_name":"Alice"}' | jq

# Get a user by ID
curl -s localhost:3001/users/<id> | jq

# List all users
curl -s localhost:3001/users | jq
```

### book-service — `localhost:3002`

```bash
# Create a book
curl -s -X POST localhost:3002/books \
  -H 'Content-Type: application/json' \
  -d '{"isbn":"9780132350884","title":"Clean Code","author":"Robert C. Martin"}' | jq

# Get a book by ID
curl -s localhost:3002/books/<id> | jq

# List all books
curl -s localhost:3002/books | jq

# Delete a book
curl -s -X DELETE localhost:3002/books/<id>
```

### review-service — `localhost:3003`

```bash
# Submit a review (rating: 1–5)
curl -s -X POST localhost:3003/reviews \
  -H 'Content-Type: application/json' \
  -d '{"book_id":"<book-uuid>","user_id":"<user-uuid>","rating":5,"body":"Excellent book!"}' | jq

# List reviews for a book
curl -s localhost:3003/books/<book-id>/reviews | jq
```

## Project structure

```
microservice-sample/
├── Cargo.toml              workspace root
├── docker-compose.yml      3 × Postgres 16
├── .env.example            DATABASE_URL defaults
├── libraries/
│   └── library-core/      UserId, BookId, ReviewId ID types
└── services/
    ├── user-service/
    ├── book-service/
    └── review-service/
```

Each service is structured as:

```
<service>/src/
├── domain/          pure entities, value objects, repository traits
├── application/     use cases, DTO conversions
├── infrastructure/  Diesel schema, models, pool, repository implementations
└── api/             Axum handlers, request/response DTOs
```

## Working with migrations

Migrations are embedded in each binary and run on startup. For manual use:

```bash
cd services/user-service   # or book-service / review-service

diesel migration run        # apply pending migrations
diesel migration redo       # rollback + reapply latest
diesel print-schema         # regenerate src/infrastructure/database/schema.rs
```

## Environment variables

See `.env.example` for all variables. Each service reads:

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | see `.env.example` | Postgres connection string |
| `BIND_ADDR` | `0.0.0.0:300x` | Address the HTTP server binds to |
