FROM rust:1.52-bullseye

RUN apt update &&\
  apt install -y libpython3.9

ENV LIBRARY_PATH /usr/lib/python3.9/config-3.9-x86_64-linux-gnu

RUN apt install -y yarn

RUN cargo install wasm-pack