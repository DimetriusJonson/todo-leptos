FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl libc-dev binaryen

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.6/cargo-leptos-installer.sh | sh

RUN rustup target add wasm32-unknown-unknown

WORKDIR /work

COPY app ./app
COPY server ./server
COPY public ./public
COPY style ./style
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY .env.docker ./.env
COPY .sqlx ./.sqlx
#COPY rust-toolchain.toml ./

RUN cargo leptos build --release -vv

FROM alpine:3.22.4 AS runner

WORKDIR /app

COPY --from=builder /work/site /app/site
COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /work/server/migrations /app/

EXPOSE 8080
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/server"]
