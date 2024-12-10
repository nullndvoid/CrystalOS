# Runs the build of CrystalOS in a Docker container. It should work now anyway. Just run "DOCKER_BUILDKIT=1 docker build . -o <output_folder>."
# There was probably a good reason to include this file in the repo but it is somewhat unclear to myself.
# I have just updated a couple things to cache dependencies and create a small container to copy the relevant build artifacts to.

FROM rustlang/rust:nightly AS build

RUN USER=root cargo new --bin CrystalOS

WORKDIR /CrystalOS

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY .cargo/config.toml .cargo/config.toml
COPY ./x86_64-CrystalOS.json ./x86_64-CrystalOS.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \ 
    rustup component add llvm-tools-preview --toolchain nightly-x86_64-unknown-linux-gnu \
    && rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu \
    && rustup update \
    && cargo install bootimage

# Second, copy the source code and build the binary again.
COPY ./src ./src

# Build in debug mode, this should maybe be parameterised for debug etc?
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/root/target \ 
    cargo build \
    && cargo bootimage --target x86_64-CrystalOS.json

# Here I will define a separate stage for exporting the final image.
# TODO: Setup a VM and run the binary on it or something cool like that. 
FROM scratch

# Copy just the binaries we need over here for a nice small final image.
COPY --from=build /CrystalOS/target/x86_64-CrystalOS/debug/CrystalOS /CrystalOS/target/x86_64-CrystalOS/debug/bootimage-CrystalOS.bin /
