FROM rustlang/rust:nightly

WORKDIR /app

RUN mkdir -p src
COPY src src
COPY Cargo.toml Cargo.lock /app/

RUN cargo build

EXPOSE 80 8000

CMD ["cargo", "run"]
