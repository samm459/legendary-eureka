FROM ubuntu:20.04

# Install Rust
RUN apt-get update && \
    apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the path to include the Rust toolchain
ENV PATH="/root/.cargo/bin:${PATH}"

# Install the necessary dependencies for building V8
RUN apt-get update && \
    apt-get install -y build-essential git pkg-config python3-dev

# Clone the V8 repository
RUN git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git

# Set the path to include the depot_tools directory
ENV PATH="/depot_tools:${PATH}"

# Fetch the V8 source code and its dependencies
RUN fetch v8

# Change to the V8 directory
WORKDIR /v8

# Build and install V8
RUN tools/dev/v8gen.py -vv x64.release -- is_component_build=true && \
    ninja -C out.gn/x64.release/

# Change back to the root directory
WORKDIR /

# Copy the Rust project files
COPY . .