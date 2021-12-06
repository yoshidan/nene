FROM rust:1.56

WORKDIR /usr/src/nene
COPY . .

RUN cargo install --path .

CMD ["nene"]