# -----------
# Chef stage
# -----------
FROM rust:1.96.0-alpine AS chef
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig perl && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install cargo-chef
WORKDIR /usr/src/app

# -----------
# Planner
# -----------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# -----------
# Builder
# -----------
FROM chef AS builder
ENV OPENSSL_STATIC=1 \
    OPENSSL_DIR=/usr
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/contizu-zelivery

# -------------
# Runtime stage
# -------------
FROM scratch
WORKDIR /app

LABEL org.opencontainers.image.description="contizu-zelivery-api mock"

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/contizu-zelivery .

EXPOSE 3000
USER 1000:1000
ENV RUST_LOG=info \
    RUST_BACKTRACE=1

ENTRYPOINT ["./contizu-zelivery"]
