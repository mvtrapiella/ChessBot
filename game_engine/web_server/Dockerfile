FROM rust:1-slim-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY game_engine/board_backend ./game_engine/board_backend
COPY game_engine/magic_numbers ./game_engine/magic_numbers
COPY game_engine/web_server ./game_engine/web_server

RUN cargo build --release -p web_server

FROM debian:bookworm-slim
WORKDIR /app

## We create and umpriviledge user beacuse for the port 0.0.0.3000 we do not need root permissions and
## we do not touch anything else on the file system. We do this in case we have a vulnerability to reduce the surface of attack
RUN useradd --system --no-create-home appuser
COPY --from=builder --chown=appuser:appuser /app/target/release/web_server ./web_server
USER appuser

EXPOSE 3000
CMD ["./web_server"]