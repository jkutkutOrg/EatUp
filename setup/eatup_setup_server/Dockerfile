FROM jkutkut/docker4rust:latest as dev

RUN apk add --update docker openrc && \
    rc-update add docker boot

# ------------------

# FROM dev as builder

# WORKDIR /app

# COPY . .

# RUN cargo build --release

# ------------------

FROM alpine:3.16 as release

WORKDIR /app
# TODO copy scripts and needed resources form local
# "/code/db/load_db.sql", "/installation/load_db.sql"

# Installation files
COPY ../../db /code # TODO this is not right

# Frontend code
COPY ../eatup_setup_front/dist /www
COPY target/release/eatup_setup_server .
ENTRYPOINT ["./eatup_setup_server"]


# TODO copy dir when installing