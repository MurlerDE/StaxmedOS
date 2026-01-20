# Security Guide

## JWT Secret Configuration

### ⚠️ CRITICAL: JWT Secret Setup

The JWT secret is used to sign and verify authentication tokens. **Never use the default value in production!**

### 1. Generate a Secure Secret

#### Option A: Using OpenSSL (Recommended)
```bash
openssl rand -base64 32
```

#### Option B: Using the provided script
```bash
cd backend
chmod +x generate-jwt-secret.sh
./generate-jwt-secret.sh
```

#### Option C: Using Node.js
```bash
node -e "console.log(require('crypto').randomBytes(32).toString('base64'))"
```

### 2. Add to .env File

Create or edit `backend/.env`:

```env
JWT_SECRET=your-generated-secret-here
```

### 3. Security Checklist

- [ ] JWT_SECRET is at least 32 characters long
- [ ] JWT_SECRET is randomly generated
- [ ] JWT_SECRET is different for each environment (dev, staging, prod)
- [ ] JWT_SECRET is stored in `.env` file (not in code)
- [ ] `.env` file is in `.gitignore`
- [ ] JWT_SECRET is never committed to version control
- [ ] Production JWT_SECRET is stored in a secure secrets manager

### 4. Best Practices

1. **Rotate Secrets Regularly**: Change your JWT secret periodically (e.g., every 90 days)
2. **Use Environment-Specific Secrets**: Never reuse the same secret across environments
3. **Store Securely**: In production, use environment variables or a secrets manager (e.g., HashiCorp Vault, AWS Secrets Manager)
4. **Monitor**: Log failed authentication attempts
5. **Token Expiration**: Tokens expire after 24 hours (configured in `auth/mod.rs`)

### 5. What Happens If Secret Is Compromised?

If your JWT secret is exposed:

1. **Immediately** generate a new secret
2. Update the `JWT_SECRET` in your environment
3. Restart the backend server
4. All existing tokens will be invalidated
5. Users will need to log in again

### 6. Configuration in Code

The JWT secret is loaded from environment variables in `config.rs`:

```rust
pub fn from_env() -> Self {
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file");
    // ...
}
```

If `JWT_SECRET` is not set, the application will **panic on startup** - this is intentional to prevent running without proper security.

## Database Security

### Connection String

Store your database connection string in `.env`:

```env
DATABASE_URL=postgresql://username:password@localhost:5432/staxmed_db
```

**Never commit database credentials to Git!**

## CORS Configuration

Current configuration allows all origins (development only):

```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
```

**TODO for Production**: Restrict CORS to your frontend domain:

```rust
let cors = CorsLayer::new()
    .allow_origin("https://your-frontend-domain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
```

## Password Hashing

**TODO**: Implement password hashing using Argon2id (dependency already included):

```rust
use argon2::{Argon2, PasswordHash, PasswordVerifier};

// Verify password
let parsed_hash = PasswordHash::new(&user.password_hash)?;
Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash)?;
```

## HTTPS / TLS

**Production Requirement**: Always use HTTPS in production!

- Deploy behind a reverse proxy (nginx, Caddy, Traefik)
- Or use Axum with `rustls` for native TLS support

## Security Headers

**TODO**: Add security headers using `tower-http`:

```rust
use tower_http::set_header::SetResponseHeaderLayer;

.layer(SetResponseHeaderLayer::if_not_present(
    header::X_CONTENT_TYPE_OPTIONS,
    HeaderValue::from_static("nosniff"),
))
.layer(SetResponseHeaderLayer::if_not_present(
    header::X_FRAME_OPTIONS,
    HeaderValue::from_static("DENY"),
))
```

## Audit Log

**Recommended**: Implement audit logging for:
- Failed login attempts
- Admin actions
- Sensitive data access
- Configuration changes

## Regular Security Updates

Keep dependencies up to date:

```bash
cargo update
cargo audit
```

Install `cargo-audit`:
```bash
cargo install cargo-audit
```

## Contact

For security issues, please contact the development team immediately.
**Do not open public GitHub issues for security vulnerabilities!**