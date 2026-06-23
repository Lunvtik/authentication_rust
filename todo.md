# Checklist A — Core Features

## HealthCheck (done)
- [x] /health_check endpoint
- [x] route registered in startup.rs
- [x] integration test via spawn_app

## PasswordHashing (core done)
- [x] Argon2id hasher with fixed parameters (compute_password_hash)
- [x] per-password salt, PHC format
- [x] verify against stored hash (verify_password)
- [x] unit tests: hash-verify, wrong password, same input yields different hashes
- [x] move code from mod.rs into hashing.rs (slice form)

## CredentialAuth (done)
- [x] secrecy::Secret for passwords in RAM
- [x] validate_credentials: single entry checks user and password
- [x] spawn_blocking so hashing does not freeze the async server
- [x] dummy-hash fallback against timing leak for unknown user
- [x] same error message for unknown user and wrong password
- [x] AuthError type
- [x] tests: valid, invalid, unknown user (unit + integration against real DB)

## Configuration
- [x] common/configuration.rs reads from environment variables
- [x] configuration/base.yaml and production.yaml, no secrets inside
- [x] secrets only from the environment via secrecy::Secret, never in code or repo
- [x] runtime validation: missing required variable aborts startup

## UserStore (done)
- [x] sqlx and Postgres connection pool in common/db.rs
- [x] migration: users table (user_id, username+email unique, password_hash)
- [x] index via UNIQUE: username (login lookup) and email; no separate index needed
- [x] versioned migrations in migrations/ (sqlx migrate, applied)

## Login
- [ ] serve login page (GET) as HTML
- [ ] process login form (POST), extract fields
- [ ] call validate_credentials, redirect on success
- [ ] reject invalid data cleanly
- [ ] split handler.rs (HTTP) from service.rs (logic)
- [ ] LoginError type
- [ ] tests: success, wrong password, unknown user

## SecureMessages
- [ ] no error text via a raw query parameter (understand XSS)
- [ ] seal error message with HMAC
- [ ] integrate flash-messages
- [ ] HTML escaping in the view
- [ ] test: tampered message is rejected

## SecureCookies
- [ ] set, read, delete cookie
- [ ] set HttpOnly, Secure, SameSite flags
- [ ] tests

## Sessions
- [ ] set up Redis as the session store
- [ ] integrate actix-session
- [ ] typed session wrapper, typed_session
- [ ] middleware: lock out unauthenticated users
- [ ] config.rs: Redis URI and session secret from the environment
- [ ] tests

## Logout
- [ ] clear session, purge (p. 379)
- [ ] redirect to start page
- [ ] test

## Registration
- [ ] registration form (GET and POST)
- [ ] validate input (email format, password minimum length)
- [ ] hash password via authentication::hashing
- [ ] create user in the DB, email unique
- [ ] reject duplicate registration cleanly
- [ ] RegError type
- [ ] tests

## PasswordChange
- [ ] form, reachable only for logged-in users
- [ ] verify old password
- [ ] new password: confirmation field and minimum length
- [ ] update hash in the DB
- [ ] cover all error paths
- [ ] tests

## LoginRateLimiter
- [ ] sliding-window counter (time window and limit)
- [ ] key per IP, optionally per account as well
- [ ] storage in Redis (fits the sessions)
- [ ] limit and window configurable per endpoint
- [ ] on exceeding, respond 429 with Retry-After
- [ ] tests

## AccountLockout
- [ ] failed-attempt counter per account
- [ ] threshold and lock window configurable
- [ ] reset counter on successful login
- [ ] locked: enumeration-safe message
- [ ] tests

## PasswordReset
- [ ] forgot-password form
- [ ] generate one-time token, store hashed only, with expiry
- [ ] send token by email
- [ ] reset form checks token: valid, not expired, single-use
- [ ] set new password, invalidate token immediately
- [ ] always respond "email sent if the account exists", no enumeration
- [ ] tests

## EmailVerification
- [ ] generate verify token at registration
- [ ] verified status flag on the user
- [ ] verify endpoint
- [ ] resend function
- [ ] define policy: unverified means restricted login
- [ ] tests

## CsrfProtection
- [ ] generate CSRF token per session
- [ ] embed token into the forms
- [ ] validate on every POST
- [ ] complements SameSite cookies, does not replace them
- [ ] tests

## TotpMfa
- [ ] generate and securely store a TOTP secret per user
- [ ] otpauth URI and QR code for the authenticator app
- [ ] verify 6-digit code (time window plus/minus 1)
- [ ] generate recovery codes, store hashed only
- [ ] hook MFA as a second step into login
- [ ] tests

## AuditTrail
- [ ] define event types (login ok and failed, reset, lockout)
- [ ] structured JSON logger
- [ ] persistence: DB table or append log
- [ ] never log passwords or secrets
- [ ] tests

## ApiKeys
- [ ] generate key (random), store only the hash
- [ ] visible prefix for identification
- [ ] scope and permissions per key
- [ ] rotation and revocation
- [ ] auth extractor for machine requests
- [ ] tests

## Rbac
- [ ] define roles enum and permissions
- [ ] map user to role in the DB
- [ ] permission check
- [ ] middleware or extractor to protect routes
- [ ] tests
