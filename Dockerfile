FROM rust:bookworm as builder_backend
WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() { println!(\"Hello world\") }" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs
RUN mkdir leaderboard
COPY ./backend/src/ ./src/
RUN touch src/main.rs
RUN cargo test --release && cargo build --release

FROM node:alpine as builder_frontend
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm install
COPY --from=builder_backend /app/bindings ./backend/bindings
COPY ./frontend/ ./frontend
RUN cd frontend && npm run build


FROM debian:bookworm-slim as runner
WORKDIR /app

EXPOSE 80
RUN apt update && apt install -y openssl nginx

COPY ./docker-start.sh .
COPY nginx.conf /etc/nginx/sites-enabled/default
COPY --from=builder_backend /app/target/release/backend ./backend
COPY --from=builder_frontend /app/frontend/dist ./frontend
CMD [ "sh", "/app/docker-start.sh" ]