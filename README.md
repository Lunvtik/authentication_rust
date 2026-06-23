# auth_rust

Session-basierter Authentifizierungs-Service in Rust (actix-web), gebaut entlang
„Zero to Production in Rust" (Kapitel 10) nach dem Vertical-Slice-Prinzip: jedes Feature
ein eigener, in sich geschlossener Ordner. Sicherheit zuerst — Argon2id-Passwort-Hashing,
Secrets aus der Umgebung, Sessions server-seitig.

## Status

Fertig und grün (cargo check + clippy; Unit-Tests bestätigt):

- [x] HealthCheck — Liveness-Endpunkt
- [x] PasswordHashing — Argon2id, Salt, PHC-Format
- [x] CredentialAuth Stufe 1 — `verify_password_hash`, `AuthError`, `Credentials` (SecretString), `spawn_blocking_with_tracing`
- [x] Configuration — `Settings`/`Environment`/`get_configuration` (config + serde), Secrets aus env, Laufzeit-Validierung
- [x] UserStore (Code) — DB-Layer (sqlx), Migration `users`, Telemetry (bunyan/json), Server-Verdrahtung

Offen:

- [ ] CredentialAuth Stufe 2 — `validate_credentials` + Dummy-Hash-Fallback (braucht DB)
- [ ] Registration — Selbstregistrierung (username + email)
- [ ] Login + SecureMessages + SecureCookies — Login-Flow, HMAC-Flash, sichere Cookies
- [ ] Sessions + Logout — Redis-Store, typsichere Session, reject-anonymous
- [ ] PasswordChange — Passwort ändern (eingeloggt)
- [ ] Sicherheits-Tests gegen den Produktionscode + HANDOFF

## Ordnerstruktur

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
│       ├── authentication/       credentials, error, hashing, validate
│       ├── health_check/
│       ├── login/                Platzhalter
│       ├── registration/         Platzhalter
│       ├── sessions/             Platzhalter
│       └── password_change/      Platzhalter
└── tests/
    └── health_check.rs
```

## Stack

Rust · actix-web · sqlx/Postgres · Redis · Argon2id · secrecy · tracing · Podman

## Setup (lokal)

```
cp .env.example .env          # APP_DATABASE__PASSWORD + APP_SESSION__SECRET setzen
podman compose up -d          # Postgres + Redis
sqlx migrate run              # users-Tabelle anlegen
cargo run                     # Server auf der Adresse aus configuration/
```

## Lizenz

MIT — siehe [LICENSE](LICENSE).
