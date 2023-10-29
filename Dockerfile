FROM rust:latest

WORKDIR  /usr/src/myapp

COPY . .

RUN cargo build --release

CMD cargo run