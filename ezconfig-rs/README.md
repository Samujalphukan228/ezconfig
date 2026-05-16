# ezconfig-rs

> Zero-boilerplate configuration loading from `.env` and environment variables in Rust.

[![Crates.io](https://img.shields.io/crates/v/ezconfig-rs.svg)](https://crates.io/crates/ezconfig-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Install

```toml
[dependencies]
ezconfig-rs = "0.1"
serde = { version = "1", features = ["derive"] }
```

---

## Usage

```rust
use ezconfig_rs::Config;
use serde::Deserialize;

#[derive(Config, Deserialize, Debug)]
struct AppConfig {
    database_url: String,        // required — error if missing
    jwt_secret: String,          // required — error if missing

    #[serde(default = "default_port")]
    port: u16,                   // optional — falls back to 8080

    #[serde(default)]
    debug: bool,                 // optional — falls back to false
}

fn default_port() -> u16 { 8080 }

fn main() {
    let config = AppConfig::load().expect("Failed to load config");
    println!("Running on port {}", config.port);
}
```

Create a `.env` file in your project root:

```env
DATABASE_URL=postgres://localhost/mydb
JWT_SECRET=supersecret
PORT=3000
DEBUG=true
```

That's it. One line. No mess.

---

## How it works

1. Reads `.env` from the current directory (silently skipped if not found)
2. Merges with real environment variables (env vars take priority over `.env`)
3. Deserializes everything into your struct using `serde`
4. Returns a typed `Result<YourConfig, ezconfig_rs::Error>`

---

## Field naming

| Struct field        | Env variable        |
|---------------------|---------------------|
| `database_url`      | `DATABASE_URL`      |
| `jwt_secret`        | `JWT_SECRET`        |
| `my_custom_name`    | `MY_CUSTOM_NAME`    |
| `port`              | `PORT`              |
| `debug`             | `DEBUG`             |

---

## Error handling

```rust
match AppConfig::load() {
    Ok(cfg) => run(cfg),
    Err(ezconfig_rs::Error::Envy(e)) => {
        eprintln!("Missing or invalid config field: {e}");
        std::process::exit(1);
    }
    Err(ezconfig_rs::Error::DotEnv(e)) => {
        eprintln!("Bad .env file: {e}");
        std::process::exit(1);
    }
}
```

| Error | When |
|-------|------|
| `ezconfig_rs::Error::Envy` | A required field is missing or has an invalid value |
| `ezconfig_rs::Error::DotEnv` | The `.env` file exists but is malformed |

---

## Comparison

| Task                  | Without ezconfig-rs  | With ezconfig-rs      |
|-----------------------|----------------------|-----------------------|
| Load config           | 30–50 lines          | 1 line                |
| Add a new field       | Edit multiple places | Add to struct         |
| Handle defaults       | Manual code          | `#[serde(default)]`  |
| Error messages        | Messy                | Clean & typed         |

---

## Dependencies

| Crate                | Purpose                         |
|----------------------|---------------------------------|
| `ezconfig-rs-derive` | Powers `#[derive(Config)]`      |
| `dotenvy`            | Reads the `.env` file           |
| `envy`               | Maps env vars into your struct  |
| `serde`              | Struct deserialization          |
| `thiserror`          | Clean error types               |

---

## Crates

| Crate                | Link                                                             |
|----------------------|------------------------------------------------------------------|
| `ezconfig-rs`        | [crates.io/crates/ezconfig-rs](https://crates.io/crates/ezconfig-rs)               |
| `ezconfig-rs-derive` | [crates.io/crates/ezconfig-rs-derive](https://crates.io/crates/ezconfig-rs-derive) |

---

## License

MIT — made by [Samujalphukan228](https://github.com/Samujalphukan228)
