//! Slice: Login (HTTP-Login-Ablauf).
//! Scharf schalten auf dem login-Branch: Submodule unten einkommentieren und in
//! src/features/mod.rs `pub mod login;` ergänzen.
//! Schritte: docs/checkliste_a_kern.md -> Login / SecureMessages / SecureCookies.

// pub mod handler;   // HTTP: GET-Formular, POST verarbeiten (dünn)
// pub mod service;   // Logik: validate_credentials + Session, ohne actix-Typen
// pub mod view;      // HTML-Seite
// pub mod flash;     // HMAC-versiegelte Fehlermeldungen
// pub mod error;     // LoginError
