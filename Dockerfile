ARG RUST_VERSION

FROM rust:${RUST_VERSION}-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:bookworm as runner
COPY --from=builder /usr/local/cargo/bin/gym-register-api /usr/local/bin/gym-register-api
ENTRYPOINT [ "gym-register-api" ]