FROM rust:1.74

WORKDIR /usr/src/air-gradient-prom-rs
COPY . .

RUN cargo install --path .

CMD ["air-gradient-prom-rs"]
