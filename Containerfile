
FROM rust:bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin jfk
WORKDIR /jfk

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./templates ./templates

# build for release
RUN rm ./target/release/deps/jfk*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY --from=build /jfk/target/release/jfk .
COPY --from=build /jfk/templates ./templates

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0

# set the startup command to run your binary
CMD ["./jfk"]
