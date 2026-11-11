# env-contract

**The next-generation environment variable contract, validator, and interactive onboarding CLI.**

Stop maintaining fragile `.env.sample` files that constantly drift out of sync with your actual codebase. `env-contract` replaces static example files with a strict, self-documenting schema (`env.contract.yaml`) that validates your environment in CI/CD and interactively generates `.env` files when a developer clones your repository.

---

## Why `env-contract`?

Traditional `.env.example` workflows are broken:
- **Schema Drift:** Developers add a variable to `.env` and forget to update `.env.example`.
- **Zero Type Safety:** Plain text files don't tell you if `PORT` must be an integer, if `NODE_ENV` is an enum, or if a field is optional.
- **Painful Onboarding:** New team members have to guess what values to put in `.env` or dig through outdated docs.
- **Accidental Secret Leaks:** Copy-pasting `.env` to create `.env.sample` often leads to committing real API keys.

`env-contract` solves this by treating your environment configuration as a **typed contract**.

---

## Core Workflow

1. **Commit `env.contract.yaml`** to your Git repository (never commit `.env`).
2. **Clone & Run `env-contract init`:** New contributors get an interactive CLI wizard that walks them through every required variable with descriptions, validation, and defaults.
3. **Pull & Run `env-contract sync`:** When teammates add new variables to the contract, `sync` detects missing keys in your local `.env` and prompts only for the new ones.
4. **Deploy & Run `env-contract check`:** Enforce strict type and presence validation in CI/CD pipelines or Docker entrypoints before your app boots.

---

## The Contract Schema (`env.contract.yaml`)

Define your environment requirements in a clean, human-readable YAML file:

```yaml
version: 1

variables:
  APP_ENV:
    type: enum
    options: ["development", "staging", "production"]
    default: "development"
    required: true
    description: "Application runtime environment"

  PORT:
    type: port
    default: 8080
    required: false
    description: "HTTP server listening port"

  DATABASE_URL:
    type: url
    required: true
    secret: true
    example: "postgres://user:pass@localhost:5432/app_db"
    description: "Primary PostgreSQL connection string"

  ENABLE_TELEMETRY:
    type: bool
    default: false
    required: false
    description: "Send anonymous usage metrics"

  STRIPE_WEBHOOK_SECRET:
    type: regex
    pattern: "^whsec_[a-zA-Z0-9]{24,}$"
    required: true
    secret: true
    description: "Stripe signing secret for webhook verification"
