FROM rust:latest

USE root

RUN apt update -y --no-reccomends

RUN wget https://garagehq.deuxfleurs.fr/_releases/v1.0.0/x86_64-unknown-linux-musl/garage