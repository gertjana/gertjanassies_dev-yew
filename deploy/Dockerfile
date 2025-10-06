# Frontend build stage
FROM rust:1.90-alpine AS frontend-builder

RUN apk add --no-cache musl-dev nodejs npm curl
RUN npm install -g sass
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY frontend ./frontend
COPY backend ./backend

WORKDIR /app/frontend

RUN trunk build --release

# Backend build stage
FROM rust:1.90-alpine AS backend-builder

RUN apk add --no-cache musl-dev upx && \
    rm -rf /var/cache/apk/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY backend/ ./backend/
COPY frontend/ ./frontend/

WORKDIR /app/backend
RUN cargo build --release --package page-stats-server
RUN strip /app/target/release/page-stats-server && \
    upx --best /app/target/release/page-stats-server

# Final runtime stage
FROM nginx:alpine

LABEL maintainer="Gertjan Assies <gertjan@example.com>"
LABEL org.opencontainers.image.authors="Gertjan Assies"
LABEL org.opencontainers.image.source="https://github.com/gertjana/gertjanassies_dev-yew"

RUN apk add --no-cache supervisor ca-certificates && \
    mkdir -p /etc/supervisor/conf.d /var/log/supervisor /var/run && \
    rm -rf /var/cache/apk/*

COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=frontend-builder /app/dist /usr/share/nginx/html
COPY --from=backend-builder /app/target/release/page-stats-server /usr/local/bin/page-stats-server
COPY deploy/supervisord.conf /etc/supervisord.conf

ENV REDIS_URL=redis://127.0.0.1:6379
ENV APP_ENV=production
ENV PAGE_STATS_PORT=3001
ENV PAGE_STATS_HOST=127.0.0.1

EXPOSE 80

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
