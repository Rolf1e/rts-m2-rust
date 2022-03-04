FROM rust:1.59.0 as back_builder

WORKDIR /usr/src/rts-m2-rust

# Install the requirements
# utilities to build the back
RUN apt-get update -y && apt-get install libpq5 -y
RUN cargo install diesel_cli --no-default-features --features postgres
# utilities to build the front
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Add the cargo files
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock

ADD rts-core/Cargo.toml rts-core/Cargo.toml
ADD rts-server/Cargo.toml rts-server/Cargo.toml
ADD rts-front/Cargo.toml rts-front/Cargo.toml

# Fetch the libraries
RUN mkdir -p rts-server/src rts-front/src rts-core/src
RUN echo "fn main() {}" > rts-server/src/main.rs
RUN echo "fn main() {}" > rts-front/src/main.rs
RUN touch rts-core/src/lib.rs
RUN cargo fetch
RUN rm -rf rts-server/src rts-front/src rts-core/src

# Add the remaining files
ADD rts-core/src rts-core/src

ADD rts-server/src rts-server/src
ADD rts-server/migrations rts-server/migrations
ADD rts-server/static rts-server/static

ADD rts-front/src rts-front/src
ADD rts-front/index.html rts-front/index.html

# Build everything
RUN cd rts-front && trunk build --release
RUN cp -r rts-front/dist/* rts-server/static/
RUN cargo build --release --bin rts-server

# Cargo install doesn't really work with workspaces
RUN cp target/release/rts-server /usr/bin/

CMD echo "=> Running migrations" && \
    cd rts-server && \
    diesel migration run && \
    cd .. && \
    echo "=> Starting server" && \
    rts-server
