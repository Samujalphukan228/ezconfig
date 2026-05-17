# ezconfig

> Zero-boilerplate configuration loading from `.env` and environment variables in Rust.

```toml
[dependencies]
ezconfig-rs = "0.1.4"
serde = { version = "1", features = ["derive"] }
```

---

## Usage

Define your config struct, derive `Config`, and call `load()`:

```rust
use ezconfig::Config;
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
4. Returns a typed `Result<YourConfig, ezconfig::Error>`

---

## Field naming

The rule is simple — struct field names map to UPPERCASE env variable names:

| Struct field        | Env variable        |
|---------------------|---------------------|
| `database_url`      | `DATABASE_URL`      |
| `jwt_secret`        | `JWT_SECRET`        |
| `my_custom_name`    | `MY_CUSTOM_NAME`    |
| `port`              | `PORT`              |
| `debug`             | `DEBUG`             |

Any name works — as long as you follow `snake_case` in the struct and `SCREAMING_SNAKE_CASE` in the env.

---

## Error handling

```rust
match AppConfig::load() {
    Ok(cfg) => run(cfg),
    Err(ezconfig::Error::Envy(e)) => {
        eprintln!("Missing or invalid config field: {e}");
        std::process::exit(1);
    }
    Err(ezconfig::Error::DotEnv(e)) => {
        eprintln!("Bad .env file: {e}");
        std::process::exit(1);
    }
}
```

| Error | When |
|-------|------|
| `ezconfig::Error::Envy` | A required field is missing or has an invalid value |
| `ezconfig::Error::DotEnv` | The `.env` file exists but is malformed |

---

## Why two crates?

Rust requires procedural macros (`#[derive(...)]`) to live in their own separate crate. So ezconfig is split into:

- **`ezconfig`** — the main library you depend on
- **`ezconfig-derive`** — the proc-macro that powers `#[derive(Config)]`

As a user you only ever add `ezconfig` to your `Cargo.toml`. The derive crate is pulled in automatically.

---

## Comparison

| Task                  | Without ezconfig     | With ezconfig          |
|-----------------------|----------------------|------------------------|
| Load config           | 30–50 lines          | 1 line                 |
| Add a new field       | Edit multiple places | Add to struct          |
| Handle defaults       | Manual code          | `#[serde(default)]`   |
| Error messages        | Messy                | Clean & typed          |

---

## Dependencies

| Crate       | Purpose                              |
|-------------|--------------------------------------|
| `dotenvy`   | Reads the `.env` file                |
| `envy`      | Maps env vars into your struct       |
| `serde`     | Struct deserialization               |
| `thiserror` | Clean error types                    |

---

## License

MIT
