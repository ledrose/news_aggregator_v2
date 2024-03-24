FROM rust:1.77
WORKDIR /usr/src

RUN USER=root cargo new backend
COPY Cargo.toml Cargo.lock /usr/src/backend/

WORKDIR /usr/src/backend
RUN cargo update --dry-run
RUN cargo build --release

COPY src /usr/src/backend/src/
COPY migrations /usr/src/backend/migrations/

RUN cargo build --release \
    && mv target/release/rust_news_aggregator_v2 /bin

WORKDIR /

EXPOSE 8080 

CMD ["/bin/rust_news_aggregator_v2"]