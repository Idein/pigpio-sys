FROM rust:1.56.0-buster

ARG user
ARG TARGET_ARCH

COPY ./raspbian-cross-build-deps .
COPY ./package-armhf.lock .

USER root
RUN apt-get update \
 && ./raspbian-cross-build-deps

USER $user
RUN rustup target add ${TARGET_ARCH}

CMD ["bash"]

