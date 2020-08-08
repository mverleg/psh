
FROM ekidd/rust-musl-builder:1.44.1 AS build

ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
RUN cargo install cargo-tree
RUN cargo install cargo-bloat

WORKDIR /app

# Add the files needed to compile dependencies.
COPY --chown=rust Cargo.toml .
COPY --chown=rust Cargo.lock .
RUN sudo chown rust:rust -R . && \
    sudo chmod g+s -R . && \
    mkdir -p src && \
    printf 'fn main() { println!("placeholder for compiling stable dependencies") }' | tee src/main.rs

# Build the dependencies (release mode).
RUN cargo build --tests --release

# Now add the actual code
COPY --chown=rust Cargo.toml Cargo.lock ./
COPY --chown=rust src src

# This makes sure things are rebuilt
RUN touch -c src/main.rs

# Build the application (release mode)
RUN cargo build --all-targets --all-features --release

# Run unit tests
RUN cargo test --release


# A find is needed here for it to work with multiple platforms (musl uses different path)
RUN find . -wholename '*/release/*' -name 'psh' -type f -executable -print -exec cp {} /app/psh \;

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM scratch

ENV RUST_BACKTRACE=1
ENV PSH_PATH=/code

WORKDIR /code

COPY --from=build /app/psh /psh

ENTRYPOINT ["/psh"]
