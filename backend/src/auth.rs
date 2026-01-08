//! Authentication and authorization module.
//!
//! This module provides JWT-based authentication and password hashing using industry-standard algorithms.
//! It demonstrates several key Rust concepts including:
//!
//! # Rust Concepts Demonstrated
//!
//! - **Traits**: Custom `FromRequestParts` implementation for authentication middleware
//! - **Async Programming**: Async trait implementation for request extraction
//! - **Error Handling**: Comprehensive error propagation using `?` operator
//! - **Cryptography**: Secure password hashing with Argon2
//! - **Type Safety**: Strong typing with UUIDs and custom types
//!
//! # Security Features
//!
//! - **Password Hashing**: Argon2id algorithm with random salts
//! - **JWT Tokens**: Signed tokens with expiration for stateless authentication
//! - **Bearer Authentication**: Standard HTTP Authorization header parsing
//!
//! # Example Usage
//!
//! ```rust,ignore
//! // Hash a password during registration
//! let password_hash = hash_password("user_password")?;
//!
//! // Create a JWT token after successful login
//! let token = create_jwt(user_id, &config.jwt_secret, 24)?;
//!
//! // Use AuthUser as a request extractor in handlers
//! async fn protected_handler(user: AuthUser) -> Response {
//!     // user.user_id is automatically extracted from the JWT token
//! }
//! ```

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

// ============================================================================
// JWT Claims
// ============================================================================

/// JWT token claims structure.
///
/// These claims are encoded into the JWT token and verified on each authenticated request.
/// Following the JWT standard (RFC 7519), we use standard claim names:
///
/// - `sub` (subject): The user ID the token is issued for
/// - `exp` (expiration): Unix timestamp when the token expires
///
/// # Token Lifecycle
///
/// 1. Created during login with [`create_jwt`]
/// 2. Sent to client in `Authorization: Bearer <token>` format
/// 3. Client includes token in every authenticated request
/// 4. Verified by [`decode_jwt`] on each request
/// 5. Expires after configured duration (default 24 hours)
///
/// # Example Token Payload
/// ```json
/// {
///   "sub": "123e4567-e89b-12d3-a456-426614174000",
///   "exp": 1704067200
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - the user ID this token represents
    /// Stored as String because JWT standard requires string subjects
    pub sub: String,

    /// Expiration time as Unix timestamp (seconds since epoch)
    /// The token becomes invalid after this time
    pub exp: i64,
}

impl Claims {
    /// Creates new JWT claims with an expiration time.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user this token represents
    /// * `expiration_hours` - How many hours until the token expires
    ///
    /// # Returns
    ///
    /// Claims struct ready to be encoded into a JWT token
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let claims = Claims::new(user_id, 24); // Token expires in 24 hours
    /// ```
    pub fn new(user_id: Uuid, expiration_hours: i64) -> Self {
        // Calculate expiration timestamp by adding hours to current time
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(expiration_hours))
            .expect("valid timestamp")
            .timestamp();

        Self {
            sub: user_id.to_string(),
            exp: expiration,
        }
    }
}

// ============================================================================
// Password Hashing
// ============================================================================

/// Hashes a password using Argon2id algorithm.
///
/// This function uses **Argon2id**, the winner of the Password Hashing Competition,
/// which is resistant to both GPU cracking attacks and side-channel attacks.
///
/// # Security Details
///
/// - **Algorithm**: Argon2id (hybrid of Argon2i and Argon2d)
/// - **Salt**: Randomly generated using cryptographically secure RNG
/// - **Output**: PHC string format containing algorithm, parameters, salt, and hash
///
/// # Arguments
///
/// * `password` - The plain text password to hash (borrowed as &str to avoid taking ownership)
///
/// # Returns
///
/// * `Ok(String)` - The hashed password in PHC format
/// * `Err(AppError)` - If hashing fails (rare, usually indicates system issues)
///
/// # Example
///
/// ```rust,ignore
/// let hash = hash_password("my_secure_password")?;
/// // hash looks like: "$argon2id$v=19$m=19456,t=2,p=1$..."
/// ```
///
/// # Rust Concepts
///
/// - **Borrowing**: Takes `&str` to avoid unnecessary string clones
/// - **Error Handling**: Returns `AppResult` for proper error propagation
/// - **Trait Usage**: Uses `PasswordHasher` trait from argon2 crate
pub fn hash_password(password: &str) -> AppResult<String> {
    // Generate a random salt using OS-provided cryptographically secure RNG
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 hasher with default parameters (balanced security/performance)
    let argon2 = Argon2::default();

    // Hash the password with the generated salt
    // Convert password to bytes, hash it, and convert result to PHC string format
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::PasswordHash) // Convert argon2 error to our AppError
        .map(|hash| hash.to_string()) // Convert PasswordHash to String
}

/// Verifies a password against a stored hash.
///
/// This function checks if a plain text password matches a previously hashed password.
/// It's used during login to authenticate users.
///
/// # Arguments
///
/// * `password` - The plain text password to verify
/// * `password_hash` - The stored hash to compare against (in PHC format)
///
/// # Returns
///
/// * `Ok(())` - Password matches the hash
/// * `Err(AppError)` - Password doesn't match or hash is invalid
///
/// # Example
///
/// ```rust,ignore
/// verify_password("user_input", &user.password_hash)?;
/// // If this succeeds, password is correct
/// ```
///
/// # Security Notes
///
/// - Timing-safe comparison (resistant to timing attacks)
/// - Returns same error for invalid hash and wrong password (prevents user enumeration)
///
/// # Rust Concepts
///
/// - **Borrowing**: Takes references to avoid moving/cloning large strings
/// - **Result Type**: Uses `()` as success type since we only care if it succeeded
/// - **Error Conversion**: Maps verification failure to authentication error
pub fn verify_password(password: &str, password_hash: &str) -> AppResult<()> {
    // Parse the stored hash from PHC format
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| AppError::PasswordHash)?;

    // Verify the password against the hash
    // This is a constant-time operation to prevent timing attacks
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Authentication("Invalid credentials".to_string()))
}

// ============================================================================
// JWT Operations
// ============================================================================

/// Creates a signed JWT token for a user.
///
/// This function generates a JSON Web Token containing the user's ID and expiration time,
/// signed with HMAC-SHA256 using the provided secret key.
///
/// # Arguments
///
/// * `user_id` - The UUID of the user this token is for
/// * `secret` - The secret key used to sign the token (from environment config)
/// * `expiration_hours` - How many hours until the token expires
///
/// # Returns
///
/// * `Ok(String)` - The complete JWT token ready to be sent to the client
/// * `Err(AppError)` - If token creation fails (e.g., invalid secret)
///
/// # Token Format
///
/// The returned token has three parts separated by dots:
/// ```text
/// <header>.<payload>.<signature>
/// eyJhbGc...  .  eyJzdWI...  .  SflKxw...
/// ```
///
/// # Example
///
/// ```rust,ignore
/// let token = create_jwt(user.id, "my-secret-key", 24)?;
/// // Client should send this in: Authorization: Bearer <token>
/// ```
///
/// # Security Notes
///
/// - Secret key should be at least 256 bits (32 bytes) for security
/// - Token is signed but not encrypted (don't include sensitive data)
/// - Token should be transmitted over HTTPS only
pub fn create_jwt(user_id: Uuid, secret: &str, expiration_hours: i64) -> AppResult<String> {
    // Create claims with user ID and expiration
    let claims = Claims::new(user_id, expiration_hours);

    // Encode claims into a JWT token
    // Uses HMAC-SHA256 algorithm by default
    encode(
        &Header::default(), // HS256 algorithm
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::Jwt) // Convert JWT error to AppError
}

/// Decodes and validates a JWT token.
///
/// This function verifies the token's signature, checks expiration,
/// and extracts the claims if valid.
///
/// # Arguments
///
/// * `token` - The JWT token string to decode
/// * `secret` - The secret key used to verify the signature
///
/// # Returns
///
/// * `Ok(Claims)` - The validated claims from the token
/// * `Err(AppError)` - If token is invalid, expired, or signature doesn't match
///
/// # Validation Checks
///
/// 1. Signature validation (token hasn't been tampered with)
/// 2. Expiration check (token hasn't expired)
/// 3. Algorithm verification (prevents algorithm substitution attacks)
///
/// # Example
///
/// ```rust,ignore
/// let claims = decode_jwt(&token, "my-secret-key")?;
/// let user_id = Uuid::parse_str(&claims.sub)?;
/// ```
pub fn decode_jwt(token: &str, secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(), // Validates signature, expiration, and algorithm
    )
    .map(|data| data.claims) // Extract just the claims from the token data
    .map_err(AppError::Jwt) // Convert JWT error to AppError
}

// ============================================================================
// Authentication Extractor
// ============================================================================

/// Custom request extractor that authenticates users via JWT.
///
/// This struct can be used as a parameter in Axum handlers to automatically
/// extract and validate the authenticated user from the request.
///
/// # How It Works
///
/// 1. Extracts `Authorization: Bearer <token>` header from request
/// 2. Validates the JWT token signature and expiration
/// 3. Extracts user ID from the token claims
/// 4. Makes user_id available to the handler
///
/// # Example Usage in Handlers
///
/// ```rust,ignore
/// async fn create_expense(
///     user: AuthUser,  // Automatically authenticates!
///     Json(data): Json<CreateExpense>,
/// ) -> AppResult<Json<Expense>> {
///     // user.user_id is guaranteed to be valid here
///     let expense = create_expense_for_user(user.user_id, data).await?;
///     Ok(Json(expense))
/// }
/// ```
///
/// # Error Responses
///
/// - `401 Unauthorized` - Missing or invalid token
/// - `500 Internal Server Error` - JWT secret not configured
///
/// # Rust Concepts Demonstrated
///
/// - **Traits**: Implements `FromRequestParts` trait for custom extraction
/// - **Async Traits**: Uses `#[async_trait]` macro for async trait methods
/// - **Generics**: Generic over state type `S` for flexibility
/// - **Error Handling**: Custom rejection type for authentication failures
pub struct AuthUser {
    /// The authenticated user's UUID
    /// This is guaranteed to be valid if the extractor succeeds
    pub user_id: Uuid,
}

/// Implementation of FromRequestParts trait for AuthUser.
///
/// This trait implementation allows Axum to automatically extract and validate
/// authentication information from incoming requests.
///
/// # Trait Bounds
///
/// - `S: Send + Sync` - State must be thread-safe (required for async handlers)
///
/// # Process Flow
///
/// 1. Extract Authorization header → 401 if missing
/// 2. Get JWT secret from environment → 500 if not configured
/// 3. Decode and validate token → 401 if invalid/expired
/// 4. Parse user ID from claims → 401 if invalid UUID
/// 5. Return AuthUser with validated user_id
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    /// Custom rejection type for authentication failures
    /// Returns HTTP status code and error message
    type Rejection = (StatusCode, String);

    /// Extracts and validates authentication from request parts.
    ///
    /// # Arguments
    ///
    /// * `parts` - The request parts (headers, method, etc.)
    /// * `_state` - Application state (unused, but required by trait)
    ///
    /// # Returns
    ///
    /// * `Ok(AuthUser)` - Successfully authenticated user
    /// * `Err((StatusCode, String))` - Authentication failure with status and message
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Step 1: Extract Authorization header with Bearer token
        // TypedHeader is an Axum extractor that parses the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing authorization header".to_string(),
                )
            })?;

        // Step 2: Get JWT secret from environment variable
        // This should be configured at startup, but we check again for safety
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT secret not configured".to_string(),
            )
        })?;

        // Step 3: Decode and validate the JWT token
        // This checks signature, expiration, and extracts claims
        let claims = decode_jwt(bearer.token(), &secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            )
        })?;

        // Step 4: Parse user ID from claims.sub (subject)
        // claims.sub is a String, convert it to UUID
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid user ID in token".to_string(),
            )
        })?;

        // Step 5: Return authenticated user
        // At this point, we have a valid, non-expired token with a valid user ID
        Ok(AuthUser { user_id })
    }
}
