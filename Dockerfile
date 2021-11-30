FROM rust:1.56.1 as builder

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/teleport-server-rust"]