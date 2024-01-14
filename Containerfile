FROM --platform=$BUILDPLATFORM rust:1.75.0-alpine3.19 as build
WORKDIR /app
RUN USER=root cargo init --bin --name jfk
RUN apk update
RUN apk add --no-cache musl-dev pkgconfig jq
ARG TARGETPLATFORM
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release --locked
RUN rm src/*.rs
RUN rm ./target/release/deps/jfk*
COPY . .
RUN cargo build --release --locked
RUN wget -O tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/download/$(wget -q -O - https://api.github.com/repos/tailwindlabs/tailwindcss/releases/latest | jq -r ".tag_name")/tailwindcss-$(echo $TARGETPLATFORM|sed 's/linux\//linux-/'|sed 's/arm\/v[-,7,6]/armv7/'|sed 's/amd64/x64/') \
    && chmod u+x tailwindcss
RUN ./tailwindcss -i ./static/css/app.css -o ./static/css/style.css --minify

FROM scratch
WORKDIR /app
COPY ./Rocket.toml .
COPY ./templates ./templates
COPY --from=build /app/target/release/jfk .
COPY --from=build /app/static ./static
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV DATABASE_URL=sqlite://jfk.sqlite?mode=rwc
CMD ["./jfk"]
