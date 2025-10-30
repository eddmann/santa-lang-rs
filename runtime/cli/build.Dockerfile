FROM joseluisq/rust-linux-darwin-builder:1.86.0 AS builder
COPY . /app
RUN cd /app && cargo build --release --bin santa-cli --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/santa-cli /
ENTRYPOINT ["./santa-cli"]
