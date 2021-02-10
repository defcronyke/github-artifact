FROM rust:slim AS build-env

WORKDIR /root/

COPY src/ src/
COPY Cargo.toml .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
  cargo build --release

# ------------------------

FROM rust:slim

# RUN apk --no-cache add ca-certificates openssl curl && \
#   update-ca-certificates

WORKDIR /root/

COPY --from=build-env /root/target/release/github-artifact .

ENV PORT 8080

CMD ["./github-artifact"]
