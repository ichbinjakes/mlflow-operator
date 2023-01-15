FROM rust:latest as builder
RUN cargo new --bin mlflow-operator
WORKDIR /mlflow-operator
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . ./
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update && apt install -y openssl && apt clean -y
COPY --from=builder /mlflow-operator/target/release/mlflow-operator /
ENTRYPOINT ["/mlflow-operator"]