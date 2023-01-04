FROM rust:alpine AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apk update && apk add musl musl-dev
RUN update-ca-certificates

ENV APP=backend APP_FILE=backend

WORKDIR /source/${APP}

COPY Cargo.toml Cargo.toml

RUN mkdir src/ && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --target=x86_64-unknown-linux-musl

RUN rm ./src/*.rs && rm ./target/x86_64-unknown-linux-musl/debug/deps/${APP_FILE}*

COPY ./src/ ./src/

RUN cargo build --target=x86_64-unknown-linux-musl


FROM rust:alpine AS publish

WORKDIR /app

ENV APP=backend

COPY --from=builder /source/${APP}/target/x86_64-unknown-linux-musl/debug/${APP} ./

ENTRYPOINT ["./backend"]