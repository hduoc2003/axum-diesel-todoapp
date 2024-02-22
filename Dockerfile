# FROM rust:1.75.0-slim as builder

# WORKDIR /app

# # Create blank project. Remember to change the project's name to match the the name of your member-crate if you don't like the name `rust-todoapp`
# RUN USER=root cargo new rust-todoapp

# # We want dependencies cached, so copy those (Cargo.toml, Cargo.lock) first
# ## PLEASE NOTE: I did not copy 'Cargo.lock' because this particular project is a workspace member without its own 'Cargo.lock'
# COPY Cargo.toml /app/rust-todoapp/

# # Set the working directory
# WORKDIR /app/rust-todoapp

# # Install 'musl-tools' to enable successful image build
# RUN apt-get -y update
# RUN apt-get -y upgrade
# RUN apt-get -y install musl-tools

# ## Install target platform (Cross-Compilation) --> Needed for Alpine
# RUN rustup target add x86_64-unknown-linux-musl

# # Install some packages for diesel
# RUN apt-get -y install libpq-dev gcc-x86-64-linux-gnu \
# && cargo install diesel_cli --no-default-features --features "postgres"

# # This is a dummy build to get the dependencies cached.
# RUN cargo build --target x86_64-unknown-linux-musl --release

# # Now copy in the rest of the src code
# COPY src /app/rust-todoapp/src/

# # Copy your 'migrations' directory and 'build.rs' file too
# # COPY migrations /app/rust-todoapp/migrations

# ## Touch 'main.rs' to prevent cached release build
# RUN touch /app/rust-todoapp/src/main.rs

# # This is the actual application build.
# RUN cargo build --target x86_64-unknown-linux-musl --release

# ##### Runtime
# FROM alpine:3.16.0 AS runtime

# # Copy application binary from image 'builder'
# COPY --from=builder /app/rust-todoapp/target/x86_64-unknown-linux-musl/release/rust-todoapp /usr/local/bin

# EXPOSE 1204

# # Run the application via 'rust-todoapp.exe'
# CMD ["/usr/local/bin/rust-todoapp"]

FROM ubuntu:22.04 as builder

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get -y install build-essential
RUN apt-get -y install curl
RUN curl -y --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup update
RUN apt-get -y install musl-dev musl-tools
RUN apt-get -y install libpq-dev
RUN apt-get -y install gcc-x86-64-linux-gnu

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features "postgres"
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.19.1 AS runtime

# Copy application binary from image 'builder'
COPY --from=builder /app/rust-demo-server/target/x86_64-unknown-linux-musl/release/rust-demo-server /usr/local/bin

EXPOSE 1204

# Run the application via 'rust-demo-server.exe'
CMD ["/usr/local/bin/rust-demo-server"]
