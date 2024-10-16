FROM ghcr.io/szinn/rust-musl-chef:1.81.0 as chef
WORKDIR /build

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip /build/target/x86_64-unknown-linux-musl/release/rust-arch

FROM ubuntu:latest@sha256:d4f6f70979d0758d7a6f81e34a61195677f4f4fa576eaf808b79f17499fd93d1 AS ubuntu
RUN groupadd --gid 8779 rust-arch && useradd -g 8779 -M -u 8779 -s /usr/sbin/nologin rust-arch
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates

# Create a new stage with a minimal image
FROM scratch
COPY --from=ubuntu /etc/passwd /etc/passwd
COPY --from=ubuntu /etc/group /etc/group
COPY --from=ubuntu /etc/ssl/ /etc/ssl/
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/rust-arch /rust-arch
USER rust-arch
ENTRYPOINT ["/rust-arch"]

LABEL org.opencontainers.image.source https://github.com/szinn/rust-arch
LABEL org.opencontainers.image.description "A rust exemplar server"
