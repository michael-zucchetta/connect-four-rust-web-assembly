# Connect Four with Rust and Web Assembly

## [Demo](https://michael-zucchetta.github.io/connect-four-rust-web-assembly/)

A screenshot of the game:

![alt tag](https://github.com/michael-zucchetta/connect-four-rust-web-assembly/blob/master/demo/ingame-screen.png?raw=true)

## Intro

I implemented this as an exercise to learn Rust and WebAssembly's world. The AI levels use Monte Carlo random plays to make a choice, while the `Minmax` levels use depth-limited adversarial search. Higher Minmax levels search more turns ahead and can take longer to choose a move.

## Getting Started

Need the following steps for setting up the environment:

* `curl https://sh.rustup.rs -sSf | sh  -s -- --channel=nightly`
* `cargo install cargo-web`
* `rustup install nightly`
* `rustup target add wasm32-unknown-emscripten`
* `rustup default nightly`

And then run:

`cargo-web start`
