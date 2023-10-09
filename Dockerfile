FROM ekidd/rust-musl-builder:stable as builder
WORKDIR /home/rust
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/app . 
EXPOSE 2784
ENTRYPOINT [ "./app" ]