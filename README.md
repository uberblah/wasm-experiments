# Basic WASM Experiments

This repo has experiments to deep-dive on the relationship between a WASM module and its runtime.

# Prerequisites

1. You must have Python 3 installed and it must be the default `python` executable for your system (`python --version` will show you your default Python version)
2. You must have `wabt` installed. See https://github.com/WebAssembly/wabt for details.
3. For the Rust package you must have `cargo` set up, and the `wasm32-unknown-unknown` runtime installed. To install it run `rustup target add wasm32-unknown-unknown`

# Running

To run the examples in one of the subfolders, just run `./build_and_run.sh`. Feel free to tweak the example and re-run.
