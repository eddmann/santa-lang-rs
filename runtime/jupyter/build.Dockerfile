FROM rust:1.70.0-bullseye AS builder
COPY . /app
RUN cd /app && cargo build --release --bin santa-jupyter --target x86_64-unknown-linux-gnu

FROM jupyter/base-notebook:x86_64-notebook-6.5.4
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/santa-jupyter /opt/santa-lang-jupyter
RUN /opt/santa-lang-jupyter install
COPY runtime/jupyter/example.ipynb $HOME/
