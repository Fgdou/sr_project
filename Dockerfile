FROM rust:bookworm as builder_backend
WORKDIR /app
COPY ./backend ./
RUN cargo test --release && cargo build --release

FROM node:alpine as builder_frontend
WORKDIR /app
COPY --from=builder_backend /app/bindings ./backend/bindings
COPY ./frontend/ ./frontend
RUN cd frontend && npm install && npm run build


FROM debian:bookworm-slim as runner
WORKDIR /app

RUN apt update && apt install -y openssl nginx

COPY ./docker-start.sh .
COPY nginx.conf /etc/nginx/conf.d/000-default.conf
COPY --from=builder_backend /app/target/release/backend ./backend
COPY --from=builder_frontend /app/frontend/dist ./frontend
CMD [ "sh", "/app/docker-start.sh" ]