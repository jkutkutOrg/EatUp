FROM jkutkut/docker4rust:latest as dev

RUN apk add --update docker openrc && \
  rc-update add docker boot

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

# Staff code
COPY staff_web/eatup_staff_web/dist/assets /www/assets
COPY staff_web/eatup_staff_web/dist/index.html /www/staff.html

# Backend code
COPY setup/eatup_setup_server/target/release/eatup_setup_server .

ENTRYPOINT ["./eatup_setup_server"]