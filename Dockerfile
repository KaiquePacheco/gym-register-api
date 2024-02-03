FROM rust:1.75.0-bookworm as builder
WORKDIR /app
COPY ./src ./src
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
RUN cargo install --path .


FROM debian:bookworm as runner
COPY --from=builder /usr/local/cargo/bin/gym-register-api /usr/local/bin/gym-register-api
ENTRYPOINT [ "gym-register-api" ]