FROM rust

RUN apt-get update && apt-get upgrade && apt-get install ufw
RUN cargo run --package rustufwprofile --example main

WORKDIR /work