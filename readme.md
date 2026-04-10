# Rusty Todoish

A humble CRUD web server in Rust<br>
_Nismara Chandra March 2026_

## Useful Commands

To refresh migrations:\
`sqlx migrate revert --target-version 0 --source ./src/infra/postgres/migrations`\
`sqlx migrate run --source ./src/infra/postgres/migrations`

To run the server:\
`cargo check`\
`cargo run`\
`cargo build`

## Project Structure

- `src/`: Source code for the project
    - `app` : Contains the core application logic, including services and use cases.
    - `infra/`: Contains infrastructure-related code, such as database interactions and web server setup.
    - `models/`: Contains data models and DTOs (Data Transfer Objects) used across the application.
    - `domain/`: Contains the core business logic and entities of the application.
        - `postgres/`: Contains code related to PostgreSQL database interactions, including migrations and repository
          implementations.
    - `rest/`: Contains code related to the REST API, including route handlers, sessions management and request/response
      models.
    - `main.rs`: The entry point of the application, where the web server is initialized and routes are defined.
    - `lib.rs`: Entry point for the library, which can be used for testing or as a module in other projects.
- `tests/`: Contains integration tests for the application, ensuring that all components work together as expected.

## Features
- Graceful shutdown with signal handling
- CDRU operations for todo items
- In-house error handling
- CORS middleware
- Role-based access control (RBAC)
- Refresh token and access token with JWT
- Session management with Redis
- PostgreSQL database transaction with SQLx
- Integrated testing with reqwest

## Endpoints

# Todo

- [x] refactor app to core