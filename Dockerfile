FROM rust:1.22.1
WORKDIR /usr/src/myapp
COPY . .
RUN rustup update nightly && rustup default nightly; 
RUN cargo build --release

FROM debian:latest
COPY --from=0 /usr/src/myapp/target/release/schani_store /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/schani_store"]
