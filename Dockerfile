FROM rust:1.47 as builder
COPY . .
RUN echo "stable" > rust-toolchain
RUN cargo build

FROM rust:1.47-slim
WORKDIR /app
COPY --from=builder /target/debug/haku-server .
EXPOSE 15671

ENTRYPOINT ["/app/haku-server"]
