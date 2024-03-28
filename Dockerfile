# Use a Rust base image
FROM rust:1.75 as builder

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the dependencies without the source code
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Create a new lightweight image without the build dependencies
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/myapp/target/release/myapp .

# Set the command to run the application
CMD ["./myapp"]
