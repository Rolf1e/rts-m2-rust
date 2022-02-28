FROM rust:1.59.0 as back_builder

WORKDIR /usr/src/myapp

# Install the requirements
# utilities to build the back
RUN apt-get update -y && apt-get install libpq5 -y
RUN cargo install diesel_cli --no-default-features --features postgres
# utilities to build the front
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Only rebuild the libraries if the Cargo.toml files changed
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY rts-core/Cargo.toml rts-core/Cargo.toml
RUN mkdir rts-core/src && touch rts-core/src/lib.rs
COPY rts-server/Cargo.toml rts-server/Cargo.toml
RUN mkdir rts-server/src && echo "fn main() {}" > rts-server/src/main.rs
COPY rts-front/Cargo.toml rts-front/Cargo.toml
RUN mkdir rts-front/src && echo "fn main() {}" > rts-front/src/main.rs
RUN cargo build --release --bin rts-server

# Remove fake libraries and mains
RUN rm rts-core/src/lib.rs rts-server/src/main.rs rts-front/src/main.rs

# Copy the actual files
COPY . .

# Build the front
RUN rustup target add wasm32-unknown-unknown # for some reason it's not enough at line 11??
RUN cd rts-front && trunk build --release
RUN cp -r rts-front/dist/* rts-server/static/

# Build and install the server
RUN cargo build --release --bin rts-server
RUN cargo install --path rts-server

CMD echo "=> Running migrations" && \
    cd rts-server && \
    diesel migration run && \
    cd .. && \
    echo "=> Starting server" && \
    rts-server
