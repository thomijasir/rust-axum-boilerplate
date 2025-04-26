# Rust Axum Boilerplate

A production-ready boilerplate for building robust web APIs and services using the [Axum](https://github.com/tokio-rs/axum) web framework in Rust. This template integrates best practices for structure, security, validation, error handling, logging, and more.

## Features

- Axum web framework integration
- Typed Multipart support
- Input validation
- Centralized error handling
- Structured logging
- Environment variable management
- YAML/JSON configuration support
- Database integration (Diesel ORM)
- Connection pooling
- Caching layer
- Formatting with `rustfmt`
- Automatic generation of Swagger/OpenAPI documentation
- Standard Logger integration
- CLEAN and SOLID code structure

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel CLI](https://diesel.rs/guides/getting-started/)
- PostgreSQL (or your configured DB)

### Installation & Running

Clone the repository and install dependencies:

```sh
git clone <repo-url>
cd axum-boilerplate
cargo build
```

Run the server:

```sh
cargo run
```

### Development: Watch & Hot Reload

```sh
cargo watch -q -c -w src/ -x run
```

### Build for Production

```sh
cargo build --release
```

## Database Migrations (Diesel)

```sh
diesel migration run
diesel migration redo
diesel migration revert
diesel migration generate <migration_name>
diesel migration create create_your_table_name
diesel print-schema > src/schema/table.rs
```

## Environment Configuration

Copy `.env.example` to `.env` and set your environment variables:

```sh
cp .env.example .env
```

## Project Structure

- `src/` - Main application source code
- `migrations/` - Diesel database migrations
- `logs/` - Log output
- `scripts/` - Utility scripts

## API Documentation

- Public API endpoints are defined in `src/routes/` and `src/controllers/`.
- Inline documentation is provided for complex logic.
- Error scenarios and handling are documented in controller files.
- **TODO:** Add OpenAPI/Swagger docs for endpoints.

## Production Readiness Checklist

### Security & Error Handling

- [ ] Remove debug `println!` statements
- [ ] Implement structured logging for all operations
- [ ] Add unique error identifiers for tracking
- [ ] Use proper error types (not generic server errors)
- [ ] Sanitize and validate all inputs

### Database & Performance

- [ ] Connection pooling optimization
- [ ] Transaction management for critical operations
- [ ] Retry mechanism for temporary failures
- [ ] Pagination for GET endpoints
- [ ] Request timeouts
- [ ] Caching where appropriate

### Input Validation & Business Logic

- [ ] Email format validation
- [ ] Input length/format validation
- [ ] Rate limiting
- [ ] Input sanitization

### Monitoring & Logging

- [ ] Audit logging for sensitive operations
- [ ] Structured logging
- [ ] Performance metrics

### Documentation

- [ ] API documentation (OpenAPI/Swagger)
- [ ] Document error scenarios and handling
- [ ] Inline documentation for complex logic

## Contribution

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

See [LICENSE](LICENSE) for details.

---

**Note:** This template provides a solid foundation but requires further hardening for full production use. See the checklist above for critical areas to address.
