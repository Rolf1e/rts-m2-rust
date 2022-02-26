FROM rust:1.58.1 as back_builder

WORKDIR /usr/src/myapp

RUN apt-get update -y && apt-get install libpq5 -y
RUN cargo install diesel_cli --no-default-features --features postgres

# Only rebuild the libraries if the Cargo.toml files changed
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY rts-core/Cargo.toml rts-core/Cargo.toml
RUN mkdir rts-core/src && touch rts-core/src/lib.rs
COPY rts-server/Cargo.toml rts-server/Cargo.toml
RUN mkdir rts-server/src && echo "fn main() {}" > rts-server/src/main.rs
COPY rts-front/Cargo.toml rts-front/Cargo.toml
RUN mkdir rts-front/src && touch rts-front/src/lib.rs
RUN cargo build --release --bin rts-server

# Remove fake libraries and mains
RUN rm rts-core/src/lib.rs rts-server/src/main.rs rts-front/src/lib.rs

# Build the actual files
COPY . .

# Build and install the server
RUN cargo build --release --bin rts-server
RUN cargo install --path rts-server

CMD echo "=> Running migrations" && \
    cd rts-server && \
    diesel migration run && \
    cd .. && \
    echo "=> Starting server" && \
    rts-server
