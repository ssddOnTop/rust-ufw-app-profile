FROM rust

RUN apt-get update && apt-get upgrade && apt-get install ufw -y

WORKDIR /work