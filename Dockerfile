FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY target/release/wasm-drydock-dev-backend /app/server
RUN chmod +x /app/server
EXPOSE 3001
CMD ["/app/server"]