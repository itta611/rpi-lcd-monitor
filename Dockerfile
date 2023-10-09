FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/rpi-lcd-monitor .
EXPOSE 2784
ENTRYPOINT [ "./rpi-lcd-monitor" ]
