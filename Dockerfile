FROM rust:latest

ARG PROTOC_VERSION

WORKDIR /usr/src/rusk
COPY . .

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip \
    && unzip protoc-3.15.8-linux-x86_64.zip -d $HOME/.local \
    && export PATH="$PATH:$HOME/.local/bin" \
    && cargo install --path . \
    && cargo clean && rm -rf $HOME/.local

EXPOSE 50051

CMD ["/usr/local/cargo/bin/server"]
