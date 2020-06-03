
FROM rustlang/rust:nightly as build

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN mkdir /app
COPY . /app
WORKDIR /app

RUN  cargo +nightly build --target=wasm32-unknown-unknown --release

# Current dev build of Envoy only supports systems with glibC
FROM ubuntu:latest

# RUN apt update && apt install -y wget

RUN mkdir /filters

# Add Envoy with WASM support built in
COPY ./envoy ./
RUN mv ./envoy /usr/local/bin/envoy && chmod +x /usr/local/bin/envoy

# Copy your compiled filter to the container
COPY --from=0 /app/target/wasm32-unknown-unknown/release/rate_limit_filter.wasm /filters/optimized.wasm