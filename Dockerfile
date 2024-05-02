FROM rust:1.77.2 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/my_axum_app .
CMD ["./my_axum_app"]