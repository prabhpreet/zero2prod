FROM rust:1.62.1
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY ./src/* ./src/
COPY ./src/routes/* ./src/routes/
COPY ./configuration/* ./configuration/
COPY ./sqlx-data.json .
COPY ./Cargo.toml .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]