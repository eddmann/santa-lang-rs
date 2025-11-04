# Dockerfile for reproducible benchmark environment
FROM rust:1.85.0-bullseye

# Install system dependencies
RUN apt-get update && apt-get install -y \
    wget \
    git \
    python3 \
    python3-pip \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install hyperfine (architecture-aware)
RUN HYPERFINE_VERSION=1.18.0 && \
    ARCH=$(dpkg --print-architecture) && \
    wget -q https://github.com/sharkdp/hyperfine/releases/download/v${HYPERFINE_VERSION}/hyperfine_${HYPERFINE_VERSION}_${ARCH}.deb && \
    dpkg -i hyperfine_${HYPERFINE_VERSION}_${ARCH}.deb && \
    rm hyperfine_${HYPERFINE_VERSION}_${ARCH}.deb

# Install Python dependencies for visualization
RUN pip3 install --no-cache-dir matplotlib numpy

# Set cargo to use fewer cores for more consistent builds
ENV CARGO_BUILD_JOBS=4
ENV CARGO_INCREMENTAL=0
ENV RUST_BACKTRACE=1

# Set up working directory
WORKDIR /workspace

# Default command
CMD ["bash"]
