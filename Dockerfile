
FROM ekidd/rust-musl-builder:1.44.1


ENV RUST_BACKTRACE=1

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-outdated
RUN cargo install cargo-audit
RUN cargo install cargo-deny
RUN cargo install cargo-tree
RUN cargo install cargo-bloat

WORKDIR /mango

# Add the files needed to compile dependencies.
COPY --chown=rust Cargo.toml .
COPY --chown=rust Cargo.lock .
RUN sudo chown rust:rust -R . && \
    sudo chmod g+s -R . && \
    mkdir -p src && \
    printf 'fn main() { println!("placeholder for compiling stable dependencies") }' | tee src/main.rs | tee src/lib.rs

# Build the code (development mode).
RUN cargo build --tests

# Build the code (release mode).
# Note: sharing dependencies between dev/release does not work yet - https://stackoverflow.com/q/59511731
RUN cargo build --tests --release
#TODO: use --out-dir if it stabilizes

# Remove Cargo.toml file, to prevent other images from forgetting to re-add it.
RUN rm -f cargo_for_coverage.sh Cargo.toml

## NOTE!
## Make sure to `touch src/main.rs` after copying source, so that everything is recompiled

# Now add the actual code
COPY --chown=rust rustfmt.toml Cargo.toml Cargo.lock ./
COPY --chown=rust src src

# This makes sure things are rebuilt
RUN bash -c 'touch -c src/main.rs; touch -c src/lib.rs'

# Build the code (debug mode)
RUN cargo build --all-targets --all-features

# Build the code (release mode)
RUN cargo build --all-targets --all-features --release

# Miscellaneous other files
COPY --chown=rust deny.toml ./






# This image builds the Mango CLI in a slim image.
# This is the image to interact with as a user of Mango.
# https://hub.docker.com/r/mangocode/mango

FROM mango_ci:stable AS build

# Probably still up-to-date, just just in case.
RUN cargo build --release

# A find is needed here for it to work with multiple platforms (musl uses different path)
RUN find . -wholename '*/release/*' -name 'mango' -type f -executable -print -exec cp {} /mango/mango_exe \;

RUN ls -als /mango/mango_exe

# Second stage image to decrease size
# Note: this version should match `base.Dockerfile`
FROM scratch

ENV RUST_BACKTRACE=1

WORKDIR /

# It's really just the executable; other files are part of the Github release, but not Docker image.
#COPY README.rst LICENSE.txt ./
COPY --from=build /mango/mango_exe /mango

#TODO @mark: maybe printf does not work in 'scratch'
#CMD printf "Welcome to the Mango docker image!\nTo use, add 'mango' after your docker run command\n"
ENTRYPOINT ["/mango"]
