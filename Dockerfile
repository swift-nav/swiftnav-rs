FROM rust:1.44-slim-stretch

ARG DEBIAN_FRONTEND=noninterative

RUN apt-get update && apt-get install -y openssh-client cmake clang libclang-7-dev

## Set-up Jenkins user
RUN useradd -u 1001 -ms /bin/bash -G staff jenkins
