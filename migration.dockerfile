FROM rust:1.75.0-slim as builder
RUN apt -y update && apt -y upgrade
RUN apt install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features "postgres"
WORKDIR /app
COPY ./migrations ./migrations
COPY migration_entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
