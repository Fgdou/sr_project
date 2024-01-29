FROM rust:bookworm as builder_backend
WORKDIR /app
COPY ./backend ./
RUN cargo test --release && cargo build --release

FROM debian:bookworm-slim as runner
WORKDIR /app

RUN apt update && apt install -y openssl

COPY --from=builder_backend /app/target/release/backend ./backend
CMD [ "/app/backend" ]