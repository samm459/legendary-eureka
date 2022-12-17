FROM ubuntu:20.04

# Install Rust and dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config zsh && \
    echo "2" | curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the path to include the Rust toolchain
ENV PATH="/root/.cargo/bin:${PATH}"

# Install V8
RUN apt-get update && \
    apt-get install -y libv8-dev

# Install cargo-edit and cargo-watch
RUN cargo install cargo-edit cargo-watch

# Change back to the root directory
WORKDIR /

# Copy the Rust project files
COPY . .
