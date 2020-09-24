FROM rust:1.44-slim-stretch

ARG DEBIAN_FRONTEND=noninterative

RUN sudo apt-get update && sudo apt-get install -y cmake clang libclang-7-dev
