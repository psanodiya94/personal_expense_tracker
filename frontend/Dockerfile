FROM rust:1.75 AS builder

# Install trunk and add wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy workspace files
COPY Cargo.toml ./
COPY frontend ./frontend

# Build the frontend
WORKDIR /app/frontend
RUN trunk build --release

# Runtime stage - serve with nginx
FROM nginx:alpine

# Copy built files to nginx
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# Copy nginx config
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
