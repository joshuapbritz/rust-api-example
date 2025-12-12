ARG RUST_VERSION=1.90.0
ARG APP_NAME=lb-interview-test

FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app


RUN apt-get update && apt-get install -y \
    clang \
    lld \
    git \
    libssl-dev \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM debian:bookworm-slim AS final

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

COPY --from=build /bin/server /bin/


EXPOSE 3030
CMD ["/bin/server"]
