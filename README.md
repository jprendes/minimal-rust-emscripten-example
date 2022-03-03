# Minimal example building Rust with Emscripten

This repository shows how to build a minimal Rust app using an emscripten backend.

## Requirements

First you need a fresh copy of [Emscripten](https://emscripten.org/docs/getting_started/downloads.html)

```bash
git clone https://github.com/emscripten-core/emsdk.git
./emsdk/emsdk install latest
./emsdk/emsdk activate latest
source ./emsdk/emsdk_env.sh
```

You will also need [rustup](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To run the example, you will need a webserver.
For this you need `npm` installed.

## Building the example

Building the example is straight forward.
```bash
cargo build --release
```

The first time it runs it might take a bit longer since it needs to setup the toolchain and build the standard libraries.

## Running the example

You can run the example serving `index.html` with a static webserver.
```bash
npx serve
```
This will print the URL where to find the webserver, usually [http://localhost:3000].
