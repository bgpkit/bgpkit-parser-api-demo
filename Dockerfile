# select build image
FROM rust:1.56 as build

# create a new empty shell project
RUN USER=root cargo new --bin bgpkit-parse-api
WORKDIR /api

# copy your source tree
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

# build for release
RUN cargo build --release

# our final base
FROM debian:buster-slim
LABEL maintainer="mingwei@bgpkit.com"
LABEL org.opencontainers.image.source="https://github.com/bgpkit/bgpkit-parser-api-demo"
LABEL org.opencontainers.image.description="BGPKIT Parser REST API demo"

RUN DEBIAN=NONINTERACTIVE apt update; apt install -y libssl-dev libpq-dev ca-certificates tzdata tini; rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /api/target/release/bgpkit-parser-api-demo ./api


# set the startup command to run your binary
ENTRYPOINT ["./api"]
