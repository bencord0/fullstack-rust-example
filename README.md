# Fullstack Rust Example

This example combines a [Yew](https://yew.rs) frontend, and an [Axum](https://docs.rs/axum)backend into a single binary serving a wasm interface.

## Setup

    $ rustup target add wasm32-unknown-unknown
    $ cargo install wasm-bindgen-cli webassembly-test-runner

## Compile

    $ cargo build

## Run

    $ cargo runserver

## Test

    $ cargo test
