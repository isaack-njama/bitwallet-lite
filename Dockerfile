# Use a Rust base image
FROM rust:1.75 as builder

# Create a new directory for the application
WORKDIR /usr/src/bitwallet

# Copy the Rust application code into the container
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:buster-slim

# Set the working directory to the location of the binary
WORKDIR /usr/src/bitwallet

# Copy the binary from the builder stage to the final image
COPY --from=builder /usr/src/bitwallet/target/release/bit_wallet_solution .

# Run the application
CMD ["./bitwallet"]
