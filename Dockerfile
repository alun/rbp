FROM debian:bullseye-slim

RUN apt update &&\
  apt install -y libpython3.9 pip &&\
  pip install pandas pandas_datareader scipy 

COPY release/service /usr/local/bin/service
COPY rpar.py /usr/local/bin

WORKDIR /usr/local/bin
CMD /usr/local/bin/service
