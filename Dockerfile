FROM rust:latest

WORKDIR  /usr/src/myapp

COPY . .

RUN cargo build --release

ENV DISCORD_TOKEN=MTE1MzE2NDY5NDQwMDU0NDg3MA.G5C1zX.552hGn50SDeY1CzqQeM_hKsAgU9ajMIE1wJm-c

CMD ./target/release/rustwithdocker