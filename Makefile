.PHONY: help setup dev-backend dev-frontend docker-up docker-down clean test

help:
	@echo "Available commands:"
	@echo "  make setup         - Install dependencies and set up the project"
	@echo "  make dev-backend   - Run backend in development mode"
	@echo "  make dev-frontend  - Run frontend in development mode"
	@echo "  make docker-up     - Start all services with Docker"
	@echo "  make docker-down   - Stop all Docker services"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make test          - Run tests"

setup:
	@echo "Installing Rust dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install trunk
	cargo install sqlx-cli
	@echo "Setting up database..."
	cd backend && cp .env.example .env
	@echo "Please edit backend/.env with your database credentials"

dev-backend:
	cd backend && cargo run

dev-frontend:
	cd frontend && trunk serve

docker-up:
	docker-compose up -d
	@echo "Application running at http://localhost:8080"

docker-down:
	docker-compose down

docker-rebuild:
	docker-compose down
	docker-compose build --no-cache
	docker-compose up -d

clean:
	cd backend && cargo clean
	cd frontend && trunk clean
	docker-compose down -v

test:
	cd backend && cargo test

migrate:
	cd backend && sqlx migrate run

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings
