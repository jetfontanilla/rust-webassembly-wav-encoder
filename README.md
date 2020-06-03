# Rust to WebAssembly WAV Encoder

WebAssembly WAV Encoder to convert raw audio and encode them to WAV files

uses the [Rust programming language](https://www.rust-lang.org) with wasm-pack for generating WASM binaries

# How to use

install the dependency from npm
```bash
$ npm install wasm-wav-encoder
```

then on your JS / TypeScript files:

```javascript
import * as wasm from "wasm-wav-encoder"


let encodedChunk = wasm.export_wav(leftBuffer, rightBuffer, sampleRate, firstChunk);
let blob = new Blob([encodedChunk], {type: "audio/wav"});


```
