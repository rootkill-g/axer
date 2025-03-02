# Axer 🪓

> [!WARNING]
> This is under active development and the APIs may change

**Function as a Service.**

**Axer 🪓** is a Serverless Function as a Service (FaaS) Platform. Enabling developers to run code without provisioning or managing servers.
It uses wasmtime runtime to execute code in response to events.
The functions can be created in any programming language that provides wasm as a compilation target, which enables developers to use the technology they are comfortable with without switching to a new programming language.

## Specification
Functions run in a lightweight and isolated and performant environment powered by wasmtime runtime, which provides near-native performance.
It utilizes WebAssembly System Interface (WASI) APIs which provide a secure standard interface for applications that can be compiled to WebAssembly (Wasm).

## APIs Provided

**Axer 🪓** provides APIs for registering wasm modules which are compiled for WASI, so make sure you embed WASI interface **(NOT JS or Web interface)**.

- [X] Register Wasm module
- [ ] Get wasm module exported functions and parameter list
- [ ] Execute wasm function
- [ ] Update wasm module
- [ ] Delete a wasm module
- [ ] ...
