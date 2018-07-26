# Connect Four with Rust and Web Assembly

## [Demo](https://michael-zucchetta.github.io/connect-four-rust-web-assembly/)

A screenshot of the game:

![alt tag](https://github.com/michael-zucchetta/connect-four-rust-web-assembly/blob/master/demo/ingame-screen.png?raw=true)

## Intro

I implemented this as an exercise to learn Rust and WebAssembly's world. The game AI is built with the Montecarlo method and the difficulty is determined by the number of moves for which the Montecarlo method is applied.

## Getting Started

Need the following steps for setting up the environment:

* `curl https://sh.rustup.rs -sSf | sh  -s -- --channel=nightly`
* `cargo install cargo-web`
* `rustup install nightly`
* `rustup target add wasm32-unknown-emscripten`
* `rustup default nightly`

And then run:

`cargo-web start`
