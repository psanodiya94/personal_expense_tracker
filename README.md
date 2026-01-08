# Personal Expense Tracker

A full-stack web application built entirely in Rust, demonstrating modern Rust concepts and real-world application development.

## 📚 Documentation

- **[SETUP.md](instructions/SETUP.md)** - Complete setup guide with step-by-step instructions
  - Prerequisites and installation
  - Database configuration
  - Running with Docker
  - Troubleshooting guide

- **[API_REFERENCE.md](instructions/API_REFERENCE.md)** - Complete API documentation
  - All endpoints with examples
  - Request/response formats
  - Authentication details
  - cURL and code examples

- **Inline Code Documentation** - Comprehensive doc comments throughout the codebase
  - Run `cargo doc --open` to view generated documentation
  - All modules, functions, and types are documented
  - Examples and usage patterns included

## ⚡ Quick Start

```bash
# Option 1: Docker (easiest)
docker-compose up -d
# Access at http://localhost:8080

# Option 2: Manual setup
# See SETUP.md for detailed instructions
cd backend && cargo run  # Terminal 1
cd frontend && trunk serve  # Terminal 2
```

## Features

### Backend (Rust + Axum)
- User authentication with JWT tokens
- Password hashing with Argon2
- RESTful API endpoints
- PostgreSQL database with SQLx
- Async/await with Tokio runtime
- Comprehensive error handling
- CRUD operations for expenses and categories
- Monthly and category-wise summaries

### Frontend (Rust + Leptos + WASM)
- Client-side rendering with WebAssembly
- Reactive UI components
- User authentication flow
- Expense management interface
- Real-time filtering by date and category
- Dashboard with summaries
- Beautiful, responsive design

## Rust Concepts Demonstrated

### Ownership & Lifetimes
- **Request Handling**: Parameters passed by value, reference, or moved
- **Database Rows**: `FromRow` trait implementation with proper ownership
- **API Models**: Zero-copy deserialization where possible

### Async / Await
- **HTTP APIs**: All endpoints use async/await pattern
- **Database Calls**: Async queries with SQLx
- **Concurrency**: Multiple database queries handled concurrently
- **WASM**: Async operations in the browser

### Traits
- **Serialization**: `Serialize` and `Deserialize` traits from serde
- **Database**: `FromRow` trait for mapping query results
- **Error Conversion**: `From` trait for error type conversions
- **Custom Extractors**: `FromRequestParts` for authentication middleware

### Error Handling
- **Result Type**: Used throughout the application
- **Custom Error Types**: `AppError` enum with `thiserror`
- **Error Propagation**: `?` operator for clean error handling
- **HTTP Error Responses**: Automatic conversion to proper status codes

### Serialization (serde)
- **JSON API**: Request/response serialization
- **Database Types**: Chrono dates, UUIDs, Decimals
- **Validation**: Input validation with `validator` crate

### Concurrency
- **Connection Pool**: Shared database pool across handlers
- **Request Handling**: Tokio runtime manages concurrent requests
- **Arc**: Shared configuration across threads

### WASM
- **Browser Execution**: Frontend compiled to WebAssembly
- **Web APIs**: DOM manipulation, LocalStorage access
- **Async in WASM**: Promises and async/await in the browser

## Tech Stack

### Backend
- **Axum**: Web framework
- **Tokio**: Async runtime
- **SQLx**: Database with compile-time query checking
- **PostgreSQL**: Relational database
- **JWT**: Authentication tokens
- **Argon2**: Password hashing

### Frontend
- **Leptos**: Reactive UI framework
- **WASM**: WebAssembly compilation
- **Trunk**: WASM build tool
- **Gloo**: Web APIs for WASM

## Project Structure

```
expense-tracker/
├── backend/
│   ├── src/
│   │   ├── main.rs              # Entry point, server setup
│   │   ├── config.rs            # Configuration management
│   │   ├── db.rs                # Database connection pool
│   │   ├── error.rs             # Custom error types
│   │   ├── auth.rs              # JWT and password handling
│   │   ├── models.rs            # Data models
│   │   ├── routes.rs            # API route definitions
│   │   └── handlers/            # Request handlers
│   │       ├── users.rs         # User auth endpoints
│   │       ├── categories.rs    # Category CRUD
│   │       ├── expenses.rs      # Expense CRUD
│   │       └── summaries.rs     # Analytics endpoints
│   └── migrations/              # SQL migrations
├── frontend/
│   ├── src/
│   │   ├── lib.rs              # Entry point
│   │   ├── api.rs              # Backend API client
│   │   ├── models.rs           # Frontend data models
│   │   └── components/         # UI components
│   │       ├── auth.rs         # Login/Register
│   │       ├── dashboard.rs    # Main dashboard
│   │       └── expense_form.rs # Add expense form
│   ├── index.html              # HTML template
│   └── style.css               # Styling
└── docker-compose.yml          # Docker setup
```

## Getting Started

### Prerequisites

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **PostgreSQL** (14+)
   ```bash
   # macOS
   brew install postgresql

   # Ubuntu/Debian
   sudo apt install postgresql postgresql-contrib
   ```

3. **Trunk** (for frontend)
   ```bash
   cargo install trunk
   ```

4. **WASM target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

### Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd personal_expense_tracker
   ```

2. **Set up the database**
   ```bash
   # Create database
   createdb expense_tracker

   # Or with psql
   psql -U postgres -c "CREATE DATABASE expense_tracker;"
   ```

3. **Configure environment variables**
   ```bash
   cd backend
   cp .env.example .env
   # Edit .env with your database credentials and JWT secret
   ```

4. **Run migrations**
   ```bash
   cd backend
   cargo install sqlx-cli
   sqlx migrate run
   ```

### Running the Application

#### Option 1: Manual (Development)

**Terminal 1 - Backend:**
```bash
cd backend
cargo run
```
Backend will start on http://localhost:3000

**Terminal 2 - Frontend:**
```bash
cd frontend
trunk serve
```
Frontend will start on http://localhost:8080

#### Option 2: Docker (Production)

```bash
docker-compose up -d
```

Access the application at http://localhost:8080

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user
- `GET /api/users/me` - Get current user (protected)

### Categories
- `GET /api/categories` - List all categories
- `POST /api/categories` - Create category
- `GET /api/categories/:id` - Get category
- `PUT /api/categories/:id` - Update category
- `DELETE /api/categories/:id` - Delete category

### Expenses
- `GET /api/expenses` - List expenses (with filters)
- `POST /api/expenses` - Create expense
- `GET /api/expenses/:id` - Get expense
- `PUT /api/expenses/:id` - Update expense
- `DELETE /api/expenses/:id` - Delete expense

### Summaries
- `GET /api/summaries/monthly` - Monthly totals
- `GET /api/summaries/categories` - Category breakdown

## Learning Resources

### Key Files for Learning

1. **Ownership & Lifetimes**: `backend/src/models.rs`, `backend/src/handlers/expenses.rs`
2. **Async/Await**: `backend/src/main.rs`, `backend/src/db.rs`
3. **Traits**: `backend/src/error.rs`, `backend/src/auth.rs`
4. **Error Handling**: All handlers in `backend/src/handlers/`
5. **Serialization**: `backend/src/models.rs`, `frontend/src/models.rs`
6. **WASM**: `frontend/src/lib.rs`, `frontend/src/api.rs`

### Concepts to Explore

- **Ownership**: Notice how data is passed between functions
- **Borrowing**: Look for `&` and `&mut` parameters
- **Lifetimes**: Check struct definitions with `'a` annotations
- **Result/Option**: Every function that can fail returns `Result`
- **Pattern Matching**: `match` statements for error handling
- **Traits**: `impl` blocks and trait bounds
- **Async**: `async fn` and `.await` throughout the codebase

## Development Tips

### Backend Development

1. **Hot Reload**: Use `cargo watch`
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

2. **Check Queries**: SQLx verifies queries at compile time
   ```bash
   cargo sqlx prepare
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

### Frontend Development

1. **Hot Reload**: Trunk automatically reloads on changes

2. **View WASM Size**
   ```bash
   trunk build --release
   ls -lh frontend/dist/*.wasm
   ```

3. **Debug in Browser**: Check console for errors

## Common Issues

### Database Connection Failed
- Ensure PostgreSQL is running
- Check `DATABASE_URL` in `.env`
- Verify database exists

### Frontend Can't Connect to Backend
- Check CORS settings in `backend/src/main.rs`
- Verify API_BASE URL in `frontend/src/api.rs`
- Ensure backend is running on port 3000

### WASM Compilation Errors
- Update Rust: `rustup update`
- Check WASM target: `rustup target list --installed`
- Clear cache: `trunk clean`

## Production Deployment

### Environment Variables
```bash
DATABASE_URL=postgres://user:pass@localhost/expense_tracker
JWT_SECRET=your-secure-256-bit-secret-key
JWT_EXPIRATION_HOURS=24
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info
```

### Build for Production

**Backend:**
```bash
cd backend
cargo build --release
./target/release/expense-tracker-backend
```

**Frontend:**
```bash
cd frontend
trunk build --release
# Serve files from frontend/dist/
```

## Contributing

This is a learning project! Feel free to:
- Add new features
- Improve error handling
- Add tests
- Enhance the UI
- Optimize performance

## License

MIT License - feel free to use this project for learning and building your own applications!

## What You'll Learn

By working through this codebase, you'll gain practical experience with:

1. **Rust Fundamentals**: Ownership, borrowing, lifetimes
2. **Async Programming**: Tokio, futures, async/await
3. **Web Development**: REST APIs, authentication, CRUD operations
4. **Database**: SQL, migrations, query building
5. **Frontend**: Reactive UI, state management, WASM
6. **Architecture**: Separation of concerns, error handling patterns
7. **Tools**: Cargo, Trunk, Docker, Git

Happy coding and learning Rust!
