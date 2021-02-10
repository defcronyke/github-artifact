FROM rust:alpine AS build-env

WORKDIR /root/

COPY src/ src/
COPY Cargo.toml .

RUN apk --no-cache add openssl-dev musl-dev && \
  cargo build --release

# ------------------------

FROM alpine:latest

RUN apk --no-cache add ca-certificates

WORKDIR /root/

COPY --from=build-env /root/target/release/github-artifact .

ENV PORT 8080

CMD ["./github-artifact"]
