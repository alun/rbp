FROM rust:1.51-bullseye

# wasm pack
RUN cargo install wasm-pack

# python
RUN apt update &&\
  apt install -y libpython3.9

ENV LIBRARY_PATH /usr/lib/python3.9/config-3.9-x86_64-linux-gnu

# node/yarn/rollup
RUN apt update &&\
  apt install -y nodejs npm &&\
  npm i -g yarn

RUN npm i -g rollup

# avoid using root
RUN groupadd -g 1000 builder &&\
  useradd -g 1000 -u 1000 -m -s /bin/bash builder 

USER builder
