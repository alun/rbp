#FROM rust:1.51-bullseye
FROM debian:bullseye

# apt packages
RUN apt update &&\
  apt install -y libpython3.9 curl nodejs npm pkg-config

ENV LIBRARY_PATH /usr/lib/python3.9/config-3.9-x86_64-linux-gnu

# node packages

RUN npm i -g yarn rollup

# avoid using root
RUN groupadd -g 1000 builder &&\
  useradd -g 1000 -u 1000 -m -s /bin/bash builder 

USER builder
WORKDIR /home/builder

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >rustup.sh
RUN sh rustup.sh -y

ENV PATH="/home/builder/.cargo/bin:${PATH}"

# wasm 
RUN /home/builder/.cargo/bin/rustup target add wasm32-unknown-unknown  &&\
  cargo install wasm-pack

# build template project to cache deps
ENV USER=builder

RUN cargo install cargo-generate

RUN cargo generate --git https://github.com/rustwasm/wasm-pack-template --name temp &&\
  cd temp &&\
  cargo build

ENV PATH="/home/builder/.cache/.wasm-pack/.wasm-bindgen-cargo-install-0.2.74/bin:${PATH}"