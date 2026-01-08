# Expense Tracker - Complete Setup Guide

This guide will walk you through setting up the Personal Expense Tracker from scratch. Follow these steps carefully to get the application running on your machine.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [System Requirements](#system-requirements)
3. [Installation Steps](#installation-steps)
4. [Database Setup](#database-setup)
5. [Backend Setup](#backend-setup)
6. [Frontend Setup](#frontend-setup)
7. [Running the Application](#running-the-application)
8. [Docker Setup (Alternative)](#docker-setup-alternative)
9. [Troubleshooting](#troubleshooting)
10. [Next Steps](#next-steps)

---

## Prerequisites

Before you begin, ensure you have the following installed on your system:

###  1. **Rust Programming Language**

Install the latest stable version of Rust using rustup:

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run: https://rustup.rs/
```

After installation, verify:

```bash
rustc --version
cargo --version
```

You should see output like:
```
rustc 1.75.0 (stable)
cargo 1.75.0
```

### 2. **PostgreSQL Database**

#### macOS (using Homebrew)
```bash
brew install postgresql@15
brew services start postgresql@15
```

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

#### Windows
Download and install from: https://www.postgresql.org/download/windows/

Verify installation:
```bash
psql --version
```

You should see output like:
```
psql (PostgreSQL) 15.x
```

### 3. **Node.js and npm** (Optional, for frontend development tools)

```bash
# macOS
brew install node

# Ubuntu/Debian
sudo apt install nodejs npm

# Windows
# Download from: https://nodejs.org/
```

### 4. **Trunk** (WASM build tool for frontend)

```bash
cargo install trunk
```

### 5. **wasm32 Target** (For compiling to WebAssembly)

```bash
rustup target add wasm32-unknown-unknown
```

### 6. **SQLx CLI** (For database migrations)

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

---

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores
- **RAM**: 4 GB
- **Disk**: 2 GB free space
- **OS**: macOS 10.15+, Ubuntu 20.04+, Windows 10+

### Recommended Requirements
- **CPU**: 4+ cores
- **RAM**: 8 GB
- **Disk**: 5 GB free space
- **OS**: Latest stable version

---

## Installation Steps

### Step 1: Clone the Repository

```bash
git clone <repository-url>
cd personal_expense_tracker
```

### Step 2: Verify Project Structure

Your project should have this structure:

```
personal_expense_tracker/
├── backend/              # Rust backend (Axum + SQLx)
│   ├── src/
│   ├── migrations/       # SQL migration files
│   └── Cargo.toml
├── frontend/             # Rust frontend (Leptos + WASM)
│   ├── src/
│   ├── index.html
│   └── Cargo.toml
├── Cargo.toml            # Workspace configuration
├── docker-compose.yml    # Docker setup
└── README.md
```

---

## Database Setup

### Step 1: Start PostgreSQL

Make sure PostgreSQL is running:

```bash
# macOS (Homebrew)
brew services start postgresql@15

# Linux
sudo systemctl start postgresql

# Check status
pg_isready
```

### Step 2: Create Database

```bash
# Connect to PostgreSQL
psql postgres

# Create database
CREATE DATABASE expense_tracker;

# Create user (optional, for production)
CREATE USER expense_user WITH PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE expense_tracker TO expense_user;

# Exit psql
\q
```

Or use the createdb utility:

```bash
createdb expense_tracker
```

### Step 3: Verify Database Creation

```bash
psql -l | grep expense_tracker
```

You should see `expense_tracker` in the list.

---

## Backend Setup

### Step 1: Navigate to Backend Directory

```bash
cd backend
```

### Step 2: Create Environment Configuration

```bash
cp .env.example .env
```

### Step 3: Edit .env File

Open `.env` in your text editor and configure:

```bash
# Database connection string
# Format: postgres://username:password@host:port/database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/expense_tracker

# JWT secret key (CHANGE THIS in production!)
# Generate a secure key with: openssl rand -base64 32
JWT_SECRET=your-super-secure-256-bit-secret-key-change-in-production

# JWT token expiration in hours
JWT_EXPIRATION_HOURS=24

# Server configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Logging level (trace, debug, info, warn, error)
RUST_LOG=info
```

**Important Security Notes:**
- The `JWT_SECRET` must be at least 256 bits (32 bytes) for security
- Never commit `.env` to version control
- Generate a secure secret with: `openssl rand -base64 32`

### Step 4: Run Database Migrations

```bash
sqlx migrate run
```

Expected output:
```
Applied 001_create_users_table.sql
Applied 002_create_categories_table.sql
Applied 003_create_expenses_table.sql
```

### Step 5: Verify Migrations

```bash
psql expense_tracker -c "\dt"
```

You should see tables:
```
        List of relations
 Schema |    Name    | Type  |  Owner
--------+------------+-------+----------
 public | categories | table | postgres
 public | expenses   | table | postgres
 public | users      | table | postgres
```

### Step 6: Build Backend

```bash
cargo build
```

This will download and compile all dependencies (may take 5-10 minutes on first run).

### Step 7: Test Backend

```bash
cargo test
```

### Step 8: Run Backend (Test)

```bash
cargo run
```

Expected output:
```
INFO  Database connection pool established
INFO  Database migrations completed
INFO  Server listening on 0.0.0.0:3000
```

Keep this terminal open. The backend should now be running on `http://localhost:3000`.

Test the health endpoint:

```bash
# In a new terminal
curl http://localhost:3000/health
```

You should see: `OK`

---

## Frontend Setup

### Step 1: Navigate to Frontend Directory

```bash
# In a new terminal
cd frontend
```

### Step 2: Configure API Endpoint

If your backend is running on a different host/port, update `frontend/src/api.rs`:

```rust
const API_BASE: &str = "http://localhost:3000/api";
```

### Step 3: Build Frontend

```bash
trunk build
```

This compiles the Rust code to WebAssembly (may take 5-10 minutes on first run).

### Step 4: Run Frontend Development Server

```bash
trunk serve
```

Expected output:
```
INFO  📦 starting build
INFO  🔨 building app
INFO  ⚙  compiling frontend to WASM
INFO  📡 serving on http://127.0.0.1:8080
```

The frontend should now be available at `http://localhost:8080`.

---

## Running the Application

### Development Mode (Two Terminals)

**Terminal 1 - Backend:**
```bash
cd backend
cargo run
```

**Terminal 2 - Frontend:**
```bash
cd frontend
trunk serve
```

**Access the application:**
- Frontend: http://localhost:8080
- Backend API: http://localhost:3000
- Health Check: http://localhost:3000/health

### Using Make Commands (Recommended)

```bash
# Start backend
make dev-backend

# In another terminal, start frontend
make dev-frontend
```

### With Hot Reloading (Development)

For automatic rebuilds on code changes:

**Backend:**
```bash
cargo install cargo-watch
cd backend
cargo watch -x run
```

**Frontend:**
```bash
cd frontend
trunk serve --watch
```

---

## Docker Setup (Alternative)

If you prefer using Docker (easiest method):

### Prerequisites for Docker

- Docker Engine 20.10+
- Docker Compose 2.0+

Install from: https://docs.docker.com/get-docker/

### Step 1: Build and Start Containers

```bash
docker-compose up -d
```

This will:
1. Start PostgreSQL container
2. Build and start backend container
3. Build and start frontend container

Expected output:
```
Creating expense_tracker_db       ... done
Creating expense_tracker_backend  ... done
Creating expense_tracker_frontend ... done
```

### Step 2: Check Container Status

```bash
docker-compose ps
```

All containers should be in "Up" state.

### Step 3: View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
```

### Step 4: Access Application

- Frontend: http://localhost:8080
- Backend: http://localhost:3000

### Step 5: Stop Containers

```bash
docker-compose down
```

### Step 6: Rebuild After Changes

```bash
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

---

## Troubleshooting

### Problem: "error: linking with `cc` failed"

**Solution:**
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt install build-essential

# Windows
# Install Visual Studio C++ Build Tools
```

### Problem: "Could not connect to database"

**Solutions:**

1. Check if PostgreSQL is running:
   ```bash
   pg_isready
   ```

2. Verify DATABASE_URL in `.env`:
   ```bash
   psql "postgres://postgres:postgres@localhost:5432/expense_tracker"
   ```

3. Check PostgreSQL logs:
   ```bash
   # macOS
   tail -f /usr/local/var/postgres/server.log

   # Linux
   sudo journalctl -u postgresql
   ```

### Problem: "JWT secret not configured"

**Solution:**
Ensure `JWT_SECRET` is set in `backend/.env`:
```bash
JWT_SECRET=your-secret-key-here
```

### Problem: "failed to resolve: use of undeclared crate or module"

**Solution:**
Clean and rebuild:
```bash
cargo clean
cargo build
```

### Problem: "error[E0554]: #![feature] may not be used on the stable release channel"

**Solution:**
Update Rust to latest stable:
```bash
rustup update stable
```

### Problem: "WASM target not found"

**Solution:**
```bash
rustup target add wasm32-unknown-unknown
```

### Problem: "trunk: command not found"

**Solution:**
```bash
cargo install trunk
```

### Problem: Frontend can't connect to backend

**Solutions:**

1. Check backend is running:
   ```bash
   curl http://localhost:3000/health
   ```

2. Check CORS settings in `backend/src/main.rs`

3. Update API_BASE in `frontend/src/api.rs`

4. Check browser console for errors (F12)

### Problem: "Port already in use"

**Solution:**

Find and kill process using the port:

```bash
# macOS/Linux
lsof -ti:3000 | xargs kill -9
lsof -ti:8080 | xargs kill -9

# Windows
netstat -ano | findstr :3000
taskkill /PID <PID> /F
```

### Problem: Migration failed

**Solution:**

Reset database:
```bash
psql postgres -c "DROP DATABASE expense_tracker;"
psql postgres -c "CREATE DATABASE expense_tracker;"
cd backend
sqlx migrate run
```

---

## Next Steps

### 1. Create Your First Account

1. Open http://localhost:8080
2. Click "Register"
3. Enter your details
4. Click "Register"

You should be automatically logged in.

### 2. Explore the Application

- Add expense categories
- Record expenses
- View monthly summaries
- Filter expenses by date/category

### 3. Explore the Code

Learn Rust concepts by reading:

- **Ownership & Lifetimes**: `backend/src/models.rs`
- **Async/Await**: `backend/src/main.rs`, `backend/src/handlers/`
- **Traits**: `backend/src/error.rs`, `backend/src/auth.rs`
- **Error Handling**: All handler files
- **WASM**: `frontend/src/lib.rs`, `frontend/src/api.rs`

### 4. Make Changes

Try modifying the code:

- Add new expense categories
- Change color schemes
- Add new fields to expenses
- Implement budget limits

### 5. Deploy to Production

See [README.md](README.md) for production deployment instructions.

---

## Additional Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [Leptos Guide](https://leptos-rs.github.io/leptos/)
- [SQLx Documentation](https://docs.rs/sqlx/)

### Community
- [Rust Community Discord](https://discord.gg/rust-lang)
- [Axum Discord](https://discord.gg/tokio)

### Tools
- **Database Client**: [pgAdmin](https://www.pgadmin.org/) or [DBeaver](https://dbeaver.io/)
- **API Testing**: [Postman](https://www.postman.com/) or [Insomnia](https://insomnia.rest/)
- **IDE**: [VS Code](https://code.visualstudio.com/) with [rust-analyzer](https://rust-analyzer.github.io/)

---

## Development Workflow

### Recommended VS Code Extensions

1. **rust-analyzer** - Rust language support
2. **CodeLLDB** - Debugging support
3. **Better TOML** - TOML syntax highlighting
4. **SQLTools** - Database management
5. **Error Lens** - Inline error display

### Useful Commands

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features

# Check code without building
cargo check

# Build optimized release version
cargo build --release

# View documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

---

## Getting Help

If you encounter issues not covered in this guide:

1. Check the [Troubleshooting](#troubleshooting) section above
2. Review error messages carefully
3. Search GitHub issues
4. Ask in Rust community forums
5. Open a new issue with:
   - Your OS and Rust version (`rustc --version`)
   - Complete error message
   - Steps to reproduce

---

**Happy Coding! 🦀**

You now have a fully functional Rust web application showcasing modern web development with Rust!
