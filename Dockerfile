# Build backend
FROM rust:1.53-alpine AS builder
WORKDIR /build
COPY . .
RUN cargo install --path . && strip /usr/local/cargo/bin/qrek

# Runtime
FROM alpine:3.13
LABEL maintainer="kb10uy"
COPY --from=builder /usr/local/cargo/bin/qrek /usr/local/bin/qrek

EXPOSE 8000
ENV LISTEN_AT="0.0.0.0:8000"
CMD ["/usr/local/bin/qrek"]
