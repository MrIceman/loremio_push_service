FROM rust:latest
RUN USER=root cargo new --bin loremio_push_service
WORKDIR /loremio_push_service
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/debug/deps/loremio_push_service*
RUN cargo build --release
CMD ["./target/release/loremio_push_service"]
