# Data to sound
A simple crate to convert data to sound, and sound to data. The sound file format is wave (.wav).
You can use it as a library or as a command line tool, also supports WASI. (dependency-free)

## Minimum supported Rust version
The minimum supported Rust version is 1.59.0.

## Note
The sound frequency is 202860Hz (202.86kHz), and the sound is mono. The sound is encoded in 16 bits.
## Disadvantages
- The wave file size limit is 4GB, so you can't store more than 4GB of data in a single file.
## Advantages
- The sound file is a standard .wav file, so you can play it with any audio player.
- The sound file will be the same size as the data file.


## Usage
There are two ways to use this crate. As a library, or as a command line tool. The library is the core of the tool, so you can use the library to create your own tool. Also we support [WASI](https://wasi.dev/), so you can use the cli in a WASI environment.
### Library
Add this to your Cargo.toml:
```toml
[dependencies]
data2sound = "0.2.0"
```
See the documentation for more information about the library.

### Command line Interface
Install the tool with cargo:
```bash
cargo install data2sound
```
Run the tool with:
```bash
data2sound --help
```

And to convert a file to sound:
```bash
data2sound encode input_file output_file.wav
```
And to convert a sound file to data:
```bash
data2sound decode input_file.wav output_file
```
### WASI
You can use the CLI in a WASI environment. All you need to do is to install the wasm file from the [releases page](https://github.com/TheAwiteb/data2sound/releases/latest), and to run it with [wasmtime](https://github.com/bytecodealliance/wasmtime) or [wasmer](https://github.com/wasmerio/wasmer).
#### Install with GitHub CLI
```bash
gh release download -R TheAwiteb/data2sound -p "*wasm32-wasi.zip"
unzip *wasm32-wasi.zip # The zip file contains the wasm
``` 
#### Wasmtime
You need to get access to the directory where the input file is located, and to the directory where the output file will be created. You can do that with the `--dir` flag.
Help message:
```bash
wasmtime data2sound.wasm -- --help
```
Encoding and decoding:
```bash
wasmtime data2sound.wasm --dir . --  encode input_file output_file.wav
wasmtime data2sound.wasm --dir . --  decode input_file.wav output_file
```
#### Wasmer
The same as with wasmtime
Help message:
```bash
wasmer data2sound.wasm -- --help
```
Encoding and decoding:
```bash
wasmer data2sound.wasm --dir . --  encode input_file output_file.wav
wasmer data2sound.wasm --dir . --  decode input_file.wav output_file
```

## Use cases
This crate can be used to store data in a sound file, and to retrieve the data from the sound file. This can be useful for storing data in a sound file.

## Benchmarks
The following benchmarks were made on a 4.600GHz 12th Gen Intel i7-12700H CPU with 16GB of RAM.
### Encoding
| File size | Audio file size | Audio length | Speed | Link |
|-----------|-----------------|------|-------| ---- |
| 2687.94MB | 2687.94MB | 01:28:13 | 1.47s | [Soundcloud-link](https://soundcloud.com/awiteb/pop-os-2204-amd64-intel-23iso) |
| 35.3MB | 35.3MB | 00:01::27 | 39.07ms | [Soundcloud-link](https://soundcloud.com/awiteb/rust-1671zip) |
## Decoding
| File size | Audio file size | Audio length | Speed | Link |
|-----------|-----------------|------|-------| ---- |
| 2687.94MB | 2687.94MB | 01:28:13 | 1.44s | [Soundcloud-link](https://soundcloud.com/awiteb/pop-os-2204-amd64-intel-23iso) |
| 35.3MB | 35.3MB | 00:01::27 | 38.97ms | [Soundcloud-link](https://soundcloud.com/awiteb/rust-1671zip) |

## Disclaimer
This tool was designed for educational purposes as it explains how to save data in an audio file. It is not recommended to exploit this thing to use cloud audio storage services to store your data, as your account may be banned.

## License
This project is licensed under the MIT license. See the LICENSE file for more information.
