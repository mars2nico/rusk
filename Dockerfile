FROM rust:latest

WORKDIR /usr/src/rusk
COPY . .

RUN cargo install --path . && rm -rf /usr/src/rusk

CMD ["server"]
