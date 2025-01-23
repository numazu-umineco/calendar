FROM rust:1.82 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release && rm -rf src

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM scratch©

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/api .

# Expose the port that the application will run on
EXPOSE 8080

# Set the entrypoint to the compiled binary
CMD ["./api"]
