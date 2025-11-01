FROM localhost/rust-musl/builder:1.91-alpine AS builder
WORKDIR /app

#RUN apk add musl-dev build-base

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

#RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS runtime

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-api-demo /rust-api-demo

EXPOSE 8080
ENTRYPOINT ["/rust-api-demo"]
