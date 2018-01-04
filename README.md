# Rust to WebAssembly WAV Encoder

This is current project is my current journey on diving deep into the [Rust programming language](https://www.rust-lang.org).
 
The aim of this project is to create WebAssembly files using the Rust compiler without using Emscripten

The project would use the microphone input using HTML5's getUserMedia() and passes the audio buffer to the WASM file which processes and encodes the buffer into a WAV file