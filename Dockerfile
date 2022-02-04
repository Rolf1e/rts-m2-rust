FROM rust:1.58.1 as back_builder

WORKDIR /usr/src/myapp

RUN apt-get update -y && apt-get install libpq5 -y
RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo install --path rts-server

CMD cd rts-server && diesel migration run && cd .. && rts-server
