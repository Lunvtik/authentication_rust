//! Slice: Sessions (eingeloggt bleiben, Server-State in Redis) + Logout.
//! Scharf schalten auf dem sessions-Branch: Submodule einkommentieren und in
//! src/features/mod.rs `pub mod sessions;` ergänzen.
//! Schritte: docs/checkliste_a_kern.md -> Sessions / Logout.

// pub mod store;          // Redis-Anbindung (actix-session)
// pub mod typed_session;  // typsichere Hülle um die Session
// pub mod middleware;     // nicht eingeloggte Nutzer aussperren; Logout
// pub mod config;         // Redis-URI, Session-Secret aus der Umgebung
// pub mod error;          // SessionError
