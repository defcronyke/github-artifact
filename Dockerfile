ARG artifact_auth

FROM rust:slim AS build-env

WORKDIR /root/

COPY src/ src/
COPY Cargo.toml .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
  cargo build --release

# ------------------------

FROM rust:slim

ARG artifact_auth

WORKDIR /root/

COPY --from=build-env /root/target/release/github-artifact .

ENV PORT 8080

ENV ARTIFACT_AUTH=$artifact_auth

CMD ["./github-artifact"]
