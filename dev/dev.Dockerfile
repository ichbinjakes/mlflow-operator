FROM rust:latest as builder
RUN cargo new mlflow-operator
WORKDIR /mlflow-operator
COPY Cargo.toml .
RUN cp ./src/main.rs ./src/crdgen.rs && cargo build
COPY . ./
RUN cargo build

FROM debian:bullseye-slim
RUN apt update && apt install -y openssl && apt clean -y
COPY --from=builder /mlflow-operator/target/debug/mlflow-operator /
ENTRYPOINT ["/mlflow-operator"]