# FROM rust:1.75.0-slim as builder
# ARG project="rust-demo-server"

# # Create blank project. Remember to change the project's name to match the the name of your member-crate if you don't like the name `rust-todoapp`
# # We want dependencies cached, so copy those (Cargo.toml, Cargo.lock) first
# ## PLEASE NOTE: I did not copy 'Cargo.lock' because this particular project is a workspace member without its own 'Cargo.lock'
# COPY Cargo.toml /app/rust-demo-server/

# # Set the working directory
# WORKDIR /app/rust-demo-server

# # Install 'musl-tools' to enable successful image build
# RUN apt-get -y update
# RUN apt-get -y upgrade
# RUN apt-get -y install musl-tools
# RUN apt install -y libpq-dev libssl-dev

# ## Install target platform (Cross-Compilation) --> Needed for Alpine
# RUN rustup target add x86_64-unknown-linux-musl


# RUN mkdir ./src
# RUN touch /app/rust-demo-server/src/main.rs

# RUN cargo build --target x86_64-unknown-linux-musl --release

# # Now copy in the rest of the src code
# COPY src /app/rust-demo-server/src/

# # Copy your 'migrations' directory and 'build.rs' file too
# # COPY migrations /app/rust-demo-server/migrations

# ## Touch 'main.rs' to prevent cached release build

# # This is the actual application build.
# RUN cargo build --target x86_64-unknown-linux-musl --release

# # RUN ls /app/rust-demo-server/target/x86_64-unknown-linux-musl/release && sleep 100s
# ##### Runtime
# FROM scratch AS runtime

# # Copy application binary from image 'builder'
# COPY --from=builder /app/rust-demo-server/target/x86_64-unknown-linux-musl/release/rust-demo-server /usr/local/bin

# EXPOSE 1204

# # Run the application via 'rust-demo-server.exe'
# CMD ["/usr/local/bin/rust-demo-server"]

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
