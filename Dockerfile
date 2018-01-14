FROM rustlang/rust:nightly
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:latest
RUN apt-get update && apt-get install -y \
  libssl1.1
COPY --from=0 /usr/src/myapp/target/release/schani_store /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/schani_store"]
