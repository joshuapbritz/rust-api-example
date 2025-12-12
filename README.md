# Rust Api Example
This example api was built based on the 2025 Luckybeard Launchpad technical challenge.

## Architecture
This api is written in [Rust](https://rust-lang.org/), using [Warp](https://docs.rs/warp/latest/warp/) for the api framework and [Diesel](https://diesel.rs/) as the database ORM.

Passwords are hashed using the Argon algorithim and jwts are being used for authentication sessions.

## Running the app (Docker)
To run this app, you will need to have Docker installed on your system. Within the repository, you can then run `docker compose up -d` to run the application. It will be expose at port `3030`.

## Running the app (Development)
To run the app in development, you will need to have `rust` and and should have `diesel` installed. You can install rust using `homebrew` or `sh`.

```bash
brew install rust
```

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You do not need to install the diesel cli, but it is very useful for developing the project further. The cli can be installed using `cargo`.

```bash
cargo install diesel_cli
```

### DB Migrations (if you installed diesel_cli)
If you install diesel cli, you can run migrations using `diesel migrations run` (runs up migrations) and you can revert migrations usings `diesel migrations redo` (runs down migrations). You can also run `diesel setup` to initialize your database if you don't want to use the db running in Docker.

Note that you will need to make sure your `.env` file has the correct database url (`DATEBASE_URL`) set.

### Running the app
To run the application for development, you will need to boot up just the Postgres db. You can do that following the steps above if you don't want to run the db in Docker. However, Docker is the simplest route, and you can run the database there by running:

```bash
docker compose up -d database
```

Once the database image is up and running, you can start your app using cargo.

```bash
cargo run
```
