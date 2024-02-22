# Simple Dockerfile, not for optimization

FROM rust:1.75.0-alpine as builder
RUN apk add openssl-dev musl-dev libpq-dev
ENV OPENSSL_DIR=/usr
WORKDIR /app

COPY . .
RUN cargo build --release

FROM scratch AS runtime
# Copy application binary from image 'builder'
COPY --from=builder /app/target/release/rust-demo-server /usr/local/bin/

EXPOSE 1204

# Run the application via 'rust-demo-server.exe'
CMD ["/usr/local/bin/rust-demo-server"]
