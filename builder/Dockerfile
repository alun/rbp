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
ENV PATH="/home/builder/.cache/.wasm-pack/.wasm-bindgen-cargo-install-0.2.74/bin:${PATH}"
ENV USER=builder

# wasm 
RUN rustup target add wasm32-unknown-unknown  &&\
  cargo install wasm-pack wasm-bindgen-cli

RUN curl -OL 'https://github.com/WebAssembly/binaryen/releases/download/version_90/binaryen-version_90-x86-linux.tar.gz' &&\
  tar xzf binaryen-version_90-x86-linux.tar.gz

ENV PATH="/home/builder/binaryen-version_90:${PATH}"