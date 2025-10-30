FROM rust:1.85.0-bullseye

# Install cross-compilation tools and zip
RUN apt-get update && \
    apt-get install -y zip gcc-x86-64-linux-gnu g++-x86-64-linux-gnu && \
    rm -rf /var/lib/apt/lists/*

# Add x86_64 target for cross-compilation
RUN rustup target add x86_64-unknown-linux-gnu

# Configure cargo to use the cross-compiler
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc
ENV CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc
ENV CXX_x86_64_unknown_linux_gnu=x86_64-linux-gnu-g++

# Copy source code
COPY . /app
WORKDIR /app

# Build the Lambda function and package it
RUN cargo build --release --bin santa-lambda --target x86_64-unknown-linux-gnu && \
    mkdir -p target/lambda/release && \
    cp target/x86_64-unknown-linux-gnu/release/santa-lambda target/lambda/release/bootstrap && \
    cd target/lambda/release && \
    zip santa-lambda.zip bootstrap
