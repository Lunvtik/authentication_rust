<div align="center">

# 🔐 auth_rust

### Session-basierter Authentifizierungs-Service in Rust

**Argon2id-Hashing · Secrets aus der Umgebung · server-seitige Sessions · Vertical Slices**

[![License](https://img.shields.io/badge/license-MIT-orange?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-ce422b?style=flat-square&logo=rust&logoColor=white)](Cargo.toml)
[![Framework](https://img.shields.io/badge/web-actix--web%204-1a73e8?style=flat-square)](https://actix.rs/)
[![Database](https://img.shields.io/badge/db-Postgres%20·%20sqlx-336791?style=flat-square&logo=postgresql&logoColor=white)](migrations/)
[![Hashing](https://img.shields.io/badge/hashing-Argon2id-7ee787?style=flat-square)](src/features/authentication/hashing.rs)
[![Status](https://img.shields.io/badge/status-work%20in%20progress-e3b341?style=flat-square)](#status)

[Status](#status) · [Architektur](#architektur) · [Setup](#setup-lokal) · [Stack](#stack) · [Sicherheit](#sicherheit)

</div>

---

`auth_rust` ist ein session-basierter Authentifizierungs-Service, gebaut entlang
*„Zero to Production in Rust"* (Kapitel 10) nach dem **Vertical-Slice-Prinzip**: jedes
Feature ist ein eigener, in sich geschlossener Ordner mit Handler, Logik, Typen und Tests.
**Sicherheit zuerst** — Passwörter werden mit Argon2id gehasht, Secrets kommen ausschließlich
aus der Umgebung, Sessions liegen server-seitig.

> **Ehrlich zum Stand.** Das Fundament steht und ist grün (`cargo check` + `clippy`,
> Unit-Tests bestätigt). Die nutzerseitigen Flows (Registrierung, Login, Sessions,
> Passwort-Änderung) sind als Slices angelegt, aber noch nicht implementiert.

## Status

**Fertig und grün** — `cargo check` + `clippy` sauber, Unit-Tests bestätigt:

| Slice | Inhalt |
|---|---|
| **HealthCheck** | Liveness-Endpunkt |
| **PasswordHashing** | Argon2id, Salt, PHC-Format |
| **CredentialAuth · Stufe 1** | `verify_password_hash`, `AuthError`, `Credentials` (SecretString), `spawn_blocking_with_tracing` |
| **Configuration** | `Settings` / `Environment` / `get_configuration` (config + serde), Secrets aus env, Laufzeit-Validierung |
| **UserStore (Code)** | DB-Layer (sqlx), Migration `users`, Telemetry (bunyan/json), Server-Verdrahtung |

**Offen:**

- [ ] **CredentialAuth · Stufe 2** — `validate_credentials` + Dummy-Hash-Fallback (braucht DB)
- [ ] **Registration** — Selbstregistrierung (username + email)
- [ ] **Login** — Login-Flow, HMAC-Flash-Messages, sichere Cookies
- [ ] **Sessions + Logout** — Redis-Store, typsichere Session, reject-anonymous
- [ ] **PasswordChange** — Passwort ändern (eingeloggt)
- [ ] **Sicherheits-Tests** gegen den Produktionscode + HANDOFF

## Architektur

Feature-first Vertical Slices unter `src/features/`. `common/` hält nur
bereichsübergreifenden Code ohne Feature-Logik — einen Slice zu löschen ist eine
Ordner-Operation.

```
auth_rust/
├── Cargo.toml
├── compose.yaml                  Postgres + Redis (Podman, rootless, gehärtet)
├── configuration/
│   ├── base.yaml
│   ├── local.yaml
│   └── production.yaml
├── migrations/
│   └── 0001_create_users.sql
├── src/
│   ├── main.rs                   Bootstrap: Config, Telemetry, Pool, Server
│   ├── lib.rs
│   ├── startup.rs                Server + Routen
│   ├── common/                   bereichsübergreifend (keine Feature-Logik)
│   │   ├── configuration.rs
│   │   ├── db.rs
│   │   └── telemetry.rs
│   └── features/                 Vertical Slices
│       ├── authentication/       credentials · error · hashing · validate
│       ├── health_check/
│       ├── registration/         Platzhalter
│       ├── login/                Platzhalter
│       ├── sessions/             Platzhalter
│       └── password_change/      Platzhalter
└── tests/
    ├── health_check.rs
    └── credential_auth.rs
```

## Setup (lokal)

```bash
cp .env.example .env          # APP_DATABASE__PASSWORD + APP_SESSION__SECRET setzen
podman compose up -d          # Postgres + Redis
sqlx migrate run              # users-Tabelle anlegen
cargo run                     # Server auf der Adresse aus configuration/
```

Tests:

```bash
cargo test                    # braucht laufendes Postgres (siehe compose.yaml)
```

## Stack

| Schicht | Wahl |
|---|---|
| Web | `actix-web` 4 |
| Datenbank | Postgres via `sqlx` (compile-time-geprüfte Queries, offline-Cache) |
| Hashing | `argon2` (Argon2id) |
| Secrets | `secrecy` — Werte nie im Klartext im Speicher-Dump |
| Telemetry | `tracing` + `tracing-bunyan-formatter` (strukturiertes JSON) |
| Sessions (geplant) | Redis |
| Laufzeit | `tokio`, Podman (rootless) |

## Sicherheit

- **Argon2id** für Passwörter — Salt pro Hash, PHC-Format.
- **Secrets nur aus der Umgebung** (`APP_DATABASE__PASSWORD`, `APP_SESSION__SECRET`) —
  nie im Code, nie im Repo. `.env` steht im `.gitignore`.
- **`secrecy::SecretString`** verhindert versehentliches Loggen von Passwörtern.
- **Gehärtetes Compose** — Postgres/Redis rootless, `cap_drop: ALL`, `no-new-privileges`,
  nur an `127.0.0.1` gebunden.
- **Laufzeit-Validierung** — fehlt eine Pflicht-Variable, bricht der Start ab.

## Lizenz

[MIT](LICENSE) — frei nutzbar, studierbar, veränderbar und teilbar.
