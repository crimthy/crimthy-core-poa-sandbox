FROM rust

WORKDIR /usr/src/rust-blockchain
COPY . .

RUN cargo install --path .

CMD ["rust-blockchain"]
