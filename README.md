# rust-learn

A workspace for learning Rust.

## Members

- [hello](hello)
- [goodbye](goodbye)

## Usage

```sh
cargo run
```

## Database

## Install SQLx CLI

```sh
cargo install sqlx-cli
```

### Create database

```sh
sqlx database create
```

#### name database from .env

### Create migrations

```sh
sqlx migrate add -r create_table_example
```

### Run migrations

```sh
sqlx migrate run --target-version 1234567
```

#### if you want to run all migrations

```sh
sqlx migrate run
```

### Revert migrations

```sh
sqlx migrate revert
```

### Drop database

```sh
sqlx database drop
```

## Redis

```sh
cargo add redis --features tokio-comp
```

## Log

```sh
cargo add env_logger
```

### complex log

```sh
cargo add log4rs
```

## SERDE (Serialization/Deserialization)

```sh
cargo add serde --features derive
```

### Serde json

```sh
cargo add serde_json
```

## Validation

```sh
cargo add validator --features derive
```
