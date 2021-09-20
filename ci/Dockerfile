FROM rust:latest as builder

RUN apt update
RUN apt install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY ./.cargo .cargo
COPY ./vendor vendor
COPY Cargo.toml Cargo.lock ./
COPY ./src src

RUN cargo install --path . --target=x86_64-unknown-linux-musl
RUN which todo-api

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/todo-api /usr/local/bin

EXPOSE 80

CMD [ "todo-api" ]