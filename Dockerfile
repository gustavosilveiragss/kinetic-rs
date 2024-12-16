FROM rust:latest

# Install mingw-w64 for Windows cross-compilation
RUN apt-get update && apt-get install -y \
    mingw-w64 \
    && rm -rf /var/lib/apt/lists/*

# Add Windows target
RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /usr/src/app

# Copy your project files
COPY . .

# Create the cargo config directory and file
RUN mkdir -p .cargo && \
    echo '[target.x86_64-pc-windows-gnu]\n\
    linker = "x86_64-w64-mingw32-gcc"\n\
    ar = "x86_64-w64-mingw32-gcc-ar"' > .cargo/config.toml

# Build for Windows
CMD ["cargo", "build", "--release", "--target", "x86_64-pc-windows-gnu"]
