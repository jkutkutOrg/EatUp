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
COPY target/release/eatup_setup_server .
ENTRYPOINT ["./eatup_setup_server"]