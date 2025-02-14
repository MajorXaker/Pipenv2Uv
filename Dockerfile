FROM rust:alpine AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:latest as runtime
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/Pipenv2Uv .
ENV DOCKER=1

CMD ["./Pipenv2Uv"]