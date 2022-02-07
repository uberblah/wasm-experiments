#!/bin/bash

rm the_answer.wasm
wat2wasm the_answer.wat
python -m http.server
