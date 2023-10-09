FROM rust:alpine as builder
WORKDIR /app
COPY . .
RUN apk add --no-cache build-base
RUN apk add --no-cache pkgconfig
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/rpi-lcd-monitor .
EXPOSE 2784
ENTRYPOINT [ "./rpi-lcd-monitor" ]
