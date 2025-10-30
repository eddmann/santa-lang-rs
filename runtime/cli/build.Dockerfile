FROM joseluisq/rust-linux-darwin-builder:1.86.0 AS builder

ARG TARGETPLATFORM
COPY . /app

RUN case "${TARGETPLATFORM}" in \
      "linux/amd64") TARGET=x86_64-unknown-linux-musl ;; \
      "linux/arm64") TARGET=aarch64-unknown-linux-musl ;; \
      *) echo "Unsupported platform: ${TARGETPLATFORM}" && exit 1 ;; \
    esac && \
    cd /app && \
    cargo build --release --bin santa-cli --target ${TARGET} && \
    mkdir -p /app/output && \
    cp /app/target/${TARGET}/release/santa-cli /app/output/santa-cli

FROM scratch
COPY --from=builder /app/output/santa-cli /
ENTRYPOINT ["./santa-cli"]
