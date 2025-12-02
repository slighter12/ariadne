# Stage 1: Build the application at debian
FROM rust:latest as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef
COPY . .
# Prepare a recipe for dependencies
RUN cargo chef prepare --recipe-path recipe.json
# Cook the dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build the application
RUN cargo build --release --bin backend

# Stage 2: Create the final, smaller image
FROM debian:12-slim as runner

WORKDIR /usr/src/app
# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend .

# Expose the port and run the application
EXPOSE 8000
CMD ["./backend"]