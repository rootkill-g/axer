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

- [X] **POST** */wasm* JSON Payload *{ "name": "module_name", "wasm": [wasm module as a byte-array] }* - Returns module registration ID and name
- [] **GET** */wasm* JSON Payload *{ "name": "module_name" }* - Returns exported functions by module and the parameters it takes
- []- **PATCH** */wasm* JSON Payload *{ "name": "module_name", "wasm": [updated wasm module to replace existing module] }* - Returns `Ok`
- []- **POST** */wasm/:id* JSON Payload *{ "name": "module_name", "function_name": "name of function exported by module", "function_parameters": "parameters that the function expects" }* - Returns the value after processing the function with passed parameters
