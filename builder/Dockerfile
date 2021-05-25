FROM rust:1.52-bullseye

# wasm pack
RUN cargo install wasm-pack

# python
RUN apt update &&\
  apt install -y libpython3.9

ENV LIBRARY_PATH /usr/lib/python3.9/config-3.9-x86_64-linux-gnu

# node/yarn
RUN apt update &&\
  apt install -y nodejs npm &&\
  npm i -g yarn