FROM rust:1.71.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

ENV PORT 8080

ENTRYPOINT ["./target/release/blog"]
