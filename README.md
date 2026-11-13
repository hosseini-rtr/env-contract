# env-contract

**The next-generation environment variable contract, validator, and interactive onboarding CLI.**

Stop maintaining fragile `.env.sample` files that constantly drift out of sync with your codebase. `env-contract` replaces static example files with a strict, self-documenting schema (`env.contract.yaml`) that manages **single or multiple environment files** (such as `.env`, `.dev.env`, `db.env`, etc.), validates them in CI/CD, and interactively generates them when a developer clones your repository.

---

## Why `env-contract`?

Traditional `.env.example` workflows break down as projects grow:
- **Multi-File Chaos:** Modern apps split configs across `.env`, `.dev.env`, `db.env`, or `auth.env`. Keeping separate `.sample` files in sync is painful and error-prone.
- **Schema Drift:** Developers add a variable to a local file and forget to document it for the rest of the team.
- **Zero Type Safety:** Plain text files don't enforce whether `PORT` is a valid integer, if `NODE_ENV` is an allowed enum, or if a field is required.
- **Accidental Secret Leaks:** Copy-pasting `.env` files to create templates often leads to committing real credentials.

`env-contract` solves this by establishing a **single, typed contract** for all your environment files.

---

## Core Workflow

1. **Commit `env.contract.yaml`** to your Git repository (keep `.env`, `.dev.env`, `db.env`, etc. in `.gitignore`).
2. **Clone & Run `env-contract init`:** New contributors get an interactive CLI wizard that walks them through every target file and variable with descriptions, live validation, and defaults.
3. **Pull & Run `env-contract sync`:** When teammates add new variables to the contract, `sync` detects missing keys across all target files and prompts only for what's new.
4. **Deploy & Run `env-contract check`:** Enforce strict type and presence validation in CI/CD pipelines or Docker entrypoints before your app boots.

---

## The Contract Schema (`env.contract.yaml`)

Define all your environment files and variables in one unified, human-readable YAML contract. Use the `target` field to route variables to specific files (defaults to `.env` if omitted):

```yaml
version: 1

# Optional: Define default target files managed by this contract
targets:
  - .env
  - .dev.env
  - db.env

variables:
  # Routed to default (.env)
  APP_ENV:
    type: enum
    options: ["development", "staging", "production"]
    default: "development"
    required: true
    description: "Application runtime environment"

  # Routed to .dev.env
  DEBUG_MODE:
    target: .dev.env
    type: bool
    default: true
    required: false
    description: "Enable verbose debug logging in local development"

  API_PORT:
    target: .dev.env
    type: port
    default: 8080
    required: true
    description: "Local HTTP server listening port"

  # Routed to db.env
  DATABASE_URL:
    target: db.env
    type: url
    required: true
    secret: true
    example: "postgres://user:pass@localhost:5432/app_db"
    description: "Primary PostgreSQL connection string"

  DB_POOL_SIZE:
    target: db.env
    type: int
    default: 10
    required: false
    description: "Maximum database connection pool size"

```

### Supported Types

| Type | Validation Rule |
| --- | --- |
| `string` | Any valid UTF-8 text |
| `int` | Valid integer (e.g., `42`, `-10`) |
| `float` | Valid floating-point number |
| `bool` | `true`, `false`, `1`, `0` |
| `port` | Integer between `1` and `65535` |
| `url` | Valid URI/URL with scheme (e.g., `https://`, `postgres://`) |
| `email` | Standard email format |
| `enum` | Must match one of the values defined in `options` |
| `regex` | Must match the regular expression defined in `pattern` |

---

## CLI Reference

### Build from source

The CLI requires Rust and Cargo. Build and run it from the repository with:

```bash
cargo build --release
./target/release/env-contract --help
```

### `env-contract init`

Smart initialization based on your workspace state:

* **If `env.contract.yaml` exists (Fresh Clone):** Launches an interactive wizard grouped by target file (`.env`, `.dev.env`, `db.env`, etc.) to generate all required env files with live type validation and masked input for `secret: true` fields.
* **If `.env` / `*.env` files exist (Existing Project):** Scans all environment files in the directory, infers variable types, flags potential secrets, maps each variable to its source `target` file, and generates a unified `env.contract.yaml` **without exposing your actual secret values**.

### `env-contract sync`

Keeps all local environment files synchronized with `env.contract.yaml`:

* Preserves existing values and comments in `.env`, `.dev.env`, `db.env`, etc.
* Creates any missing target files automatically.
* Prompts interactively **only** for newly added variables in the contract.
* Warns about orphan variables in local files that are no longer defined in the contract.

### `env-contract check`

Validates all target environment files (or a specific target) against `env.contract.yaml`:

```bash
# Check all target files defined in the contract
env-contract check

# Check a specific target file only
env-contract check --target db.env

```

* Exits with code `0` if valid, or `1` with a file-by-file error report if any variable is missing or fails validation.

### Global Flags

* `-c, --contract <path>`: Specify custom contract path (default: `./env.contract.yaml`)
* `-t, --target <file>`: Scope command execution to a specific env file (e.g., `db.env`)
* `--ci`: Run in non-interactive mode (fails immediately if required values are missing)

---

## License

MIT
