FROM rust:1.83-bullseye

WORKDIR /build

COPY Cargo.toml Cargo.lock /build/
RUN mkdir -p benches && touch benches/tree_bench.rs
RUN cargo fetch


COPY . /build/
RUN cargo build --target-dir bin/

RUN ls -l bin


