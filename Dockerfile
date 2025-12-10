ARG RUST_VERSION=1.90.1
ARG APP_NAME=rs_service_cloud


#* --- Build Stage ---
FROM rust:${RUST_VERSION} AS builder

WORKDIR /usr/src/app

# Copy actual source and build (except file in dockerignore)
COPY . .

RUN cargo build --release


#* --- Runtime Stage ---
# FROM debian:bookworm-slim AS run
FROM debian:trixie-slim AS runner
ARG APP_NAME

RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/* && apt-get clean

# Copy the statically built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/$APP_NAME /usr/local/bin/app

# Copy file configs DB and .env
COPY --from=builder /usr/src/app/config*.* /usr/local/bin

WORKDIR /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]
