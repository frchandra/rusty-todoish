# Rusty Todoish

A humble CRUD web server in Rust

## Useful commands

To refresh migrations:\
`sqlx migrate revert --target-version 0 --source ./src/infra/postgres/migrations`\
`sqlx migrate run --source ./src/infra/postgres/migrations`

# To do
- [x] refactor UserHandler to AuthHandler