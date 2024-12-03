# Template Generator

A generator for Architectury mod templates with a web UI and an interactive command line UI.

## Requirements

- Rust + cargo
- Web target:
  - [wasm-pack](https://rustwasm.github.io/wasm-pack/)
  - Java for running Gradle
  - Python for running local test web server

## Building (web)

Run `./gradlew buildWeb`. The output will be in `build/web`.

The local test web server can be launched with `./gradlew runTestServer` (requires Python). While running, this web server can be viewed at http://localhost:8000.

## Building (native)

Run `cargo build`.
