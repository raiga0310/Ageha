FROM rust AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src

RUN rm -f target/release/deps/weather*

RUN cargo build --release

ENV PORT 8080

ENTRYPOINT ["target/release/weather"]