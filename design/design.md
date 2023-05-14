# WASM Composition

## Goal

Identify a way that WASM modules can be automatically, procedurally composed at runtime.

The story would be something like this:
1. Write a WASM module to run your application
2. Identify dependency WASM modules that provide different utilities
3. Your dependency WASM modules may themselves require utility modules
4. We define the interactions between them via interfaces
5. When we launch the application, we specify the composition of all the modules -- which ones play which role in which other modules, including how many instances of each module will be in place
6. In the long run, these become an actor system made of WASM modules (and nothing else)

WASM Cloud is an existing system that aims in this direction, but I am writing this design to evaluate to what extent it satisfies my needs.

## Context

1. There is an existing proposal from WASI for a "component model" that allows flexible / runtime modularity
2. ~~There is an existing system wasmcloud aiming to do this pre-component-model~~
3. ~~There are existing smart contract runtimes that use WASM and are building their own wrappers, but they always expect contracts to bundle all of their dependencies~~

## References

1. [Component Model for WASM](https://github.com/WebAssembly/component-model)
1. [Interface Types](https://github.com/WebAssembly/interface-types/blob/main/proposals/interface-types/Explainer.md)
2. [Interface Types -- Chromium Implementation Status](https://chromestatus.com/feature/6219189974990848)
3. [Interface Types were expected in 2021 but not implemented](https://platform.uno/blog/the-state-of-webassembly-2020-and-2021/)
4. [wasmcloud's module binding implementation](https://github.com/wasmCloud/weld)
5. [cosmwasm's runtime](https://github.com/CosmWasm/cosmwasm)
