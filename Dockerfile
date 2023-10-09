FROM rust:alpine as builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    apk add --no-cache musl-dev && \
    cargo build --release && \
    rm -rf src
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/rpi-lcd-monitor .
EXPOSE 2784
ENTRYPOINT [ "./rpi-lcd-monitor" ]
