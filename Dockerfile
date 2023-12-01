FROM ubuntu:latest

RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y

RUN mkdir /src
WORKDIR /src

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup toolchain install nightly
RUN rustup install nightly-2023-08-18
RUN rustup override set nightly
RUN rustup default nightly
RUN rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN cargo install bootimage

COPY . .

RUN cargo update -p proc-macro2

CMD ["cargo", "build", "--release"]
CMD ["cargo", "test"]
