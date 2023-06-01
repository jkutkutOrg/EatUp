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
# * This should be run from the root of the project

RUN apk add --update docker openrc && \
    rc-update add docker boot; \
    rm -rf /var/cache/apk/*

WORKDIR /app

# Installation files
COPY db /code

# Frontend code
COPY setup/eatup_setup_front/dist /www

COPY setup/eatup_setup_server/target/release/eatup_setup_server .
ENTRYPOINT ["./eatup_setup_server"]