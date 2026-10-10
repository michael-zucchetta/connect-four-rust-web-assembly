# Connect Four with Rust and Web Assembly

## [Demo](https://michael-zucchetta.github.io/connect-four-rust-web-assembly/)

A screenshot of the game:

![alt tag](https://github.com/michael-zucchetta/connect-four-rust-web-assembly/blob/master/demo/ingame-screen.png?raw=true)

## Intro

I implemented this as an exercise to learn Rust and WebAssembly's world. The AI levels use Monte Carlo random plays to make a choice, while the `Minimax` levels use depth-limited adversarial search. Higher Minimax levels search more turns ahead and can take longer to choose a move.

### AI difficulty

The standard difficulty levels use Monte Carlo. Each level runs the following number of simulations for each move:

| Difficulty | Simulations |
| --- | ---: |
| Very Easy | 80 |
| Easy | 150 |
| Medium | 500 |
| Hard | 2,000 |
| Very Hard | 5,000 |
| Extreme | 10,000 |

All six Minimax difficulty levels use Minimax search and search this many plies (individual player moves) for each move:

| Difficulty | Search depth |
| --- | ---: |
| Very Easy | 1 |
| Easy | 2 |
| Normal | 3 |
| Hard | 4 |
| Very Hard | 5 |
| Extreme | 6 |

## Getting Started

Need the following steps for setting up the environment:

* `curl https://sh.rustup.rs -sSf | sh  -s -- --channel=nightly`
* `cargo install cargo-web`
* `rustup install nightly`
* `rustup target add wasm32-unknown-emscripten`
* `rustup default nightly`

And then run:

`cargo-web start`
