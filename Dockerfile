FROM rust:slim-bookworm AS chef
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y pkg-config curl && rm -rf /var/lib/apt/lists/*
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN wasm-pack build --target web --release frontend/
RUN cargo install grass && grass frontend/styles/screen.scss frontend/styles/screen.css
RUN cargo build --release --features embed-assets -p wasm-drydock-dev-backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/wasm-drydock-dev-backend /app/server
RUN chmod +x /app/server
EXPOSE 3001
CMD ["/app/server"]