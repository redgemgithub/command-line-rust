FROM rust:1.77-bookworm

WORKDIR /work
RUN rustup component add rustfmt
