FROM rust:1.76.0

WORKDIR /app

COPY . .

RUN cargo install --path .

RUN ls -a

ENTRYPOINT ["/usr/local/cargo/bin/rust_parser","samples/sample_1"]
