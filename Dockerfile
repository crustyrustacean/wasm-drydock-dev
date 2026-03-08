FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config curl && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install wasm32 target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .

# Build frontend
RUN wasm-pack build --target web --release frontend/

# Compile SCSS
RUN cargo install grass-cli && grass frontend/styles/screen.scss frontend/styles/screen.css

# Build backend with embedded assets
RUN cargo build --release --features embed-assets -p wasm-drydock-dev-backend

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wasm-drydock-dev-backend /app/server

RUN chmod +x /app/server

EXPOSE 3001

CMD ["/app/server"]