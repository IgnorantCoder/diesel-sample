FROM rust:1.34.0

RUN cargo install diesel_cli

WORKDIR /tmp/sample

COPY Cargo.lock         /tmp/sample/Cargo.lock
COPY Cargo.toml         /tmp/sample/Cargo.toml
RUN mkdir /tmp/sample/src
RUN echo 'fn main() {}'>/tmp/sample/src/main.rs
RUN cargo build
COPY src                /tmp/sample/src
COPY diesel.toml        /tmp/sample/diesel.toml   
COPY migrations         /tmp/sample/migrations
COPY start.sh           /tmp/sample/start.sh
RUN find . -name *.rs|xargs touch
RUN cargo build

ENV RUST_BACKTRACE 1

EXPOSE 8080

ENTRYPOINT ["/tmp/sample/start.sh"]