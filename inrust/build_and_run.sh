#!/bin/bash

cargo build
wasm2wat target/wasm32-unknown-unknown/debug/inrust.wasm > inrust.wat
python -m http.server
