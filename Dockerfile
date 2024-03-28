# Use the official Rust Docker image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/main

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the dependencies without the source code to cache dependencies
RUN mkdir src && echo "fn main() {println!(\"dummy\")}" > src/main.rs \
    && cargo build --release \
    && rm -f src/main.rs

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Create a new lightweight container to run the application
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/main

# Copy the compiled executable from the builder stage to the new container
COPY --from=builder /usr/src/main/target/release/main .

# Expose the port that the application listens on
EXPOSE 8080

# Command to run the application
CMD ["./main"]
