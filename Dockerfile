FROM debian:buster-slim

RUN apt update &&\
  apt install -y libpython3.9 pip &&\
  pip install pandas numpy datetime scipy pandas_datareader yfinance

COPY release/service /usr/local/bin/service
COPY rpar.py /usr/local/bin

WORKDIR /usr/local/bin
CMD /usr/local/bin/service
