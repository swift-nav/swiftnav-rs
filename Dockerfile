FROM rust:1.44-slim-buster

ARG DEBIAN_FRONTEND=noninterative

RUN apt-get update \
    && apt-get install -y openssh-client git cmake clang libclang-7-dev \
    && rustup component add rustfmt \
    && rustup component add clippy

## Set-up Jenkins user
RUN useradd -u 1001 -ms /bin/bash -G staff jenkins
