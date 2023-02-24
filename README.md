# Data to sound
A simple crate to convert data to sound, and sound to data. The sound file format is .wav.
You can use it as a library or as a command line tool.

## Minimum supported Rust version
The minimum supported Rust version is 1.56.1.

## Note
The sound frequency is 202860Hz (202.86kHz), and the sound is mono. The sound is encoded in 16 bits.
## Disadvantages
- The wave file size limit is 4GB, so you can't store more than 4GB of data in a single file.
- The decoding speed is slow, so it will take a long time to decode a large file. (see benchmarks)
## Advantages
- The sound file is a standard .wav file, so you can play it with any audio player.
- The sound file will be the same size as the data file.


## Usage
### Library
Add this to your Cargo.toml:
```toml
[dependencies]
data2sound = "0.1.0"
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

## Use cases
### Infinite storage
You can use this tool to store your data in soundcloud, or any other sound hosting service. The data is stored in the sound file, so you can store as much data as you want. The only limit is the size of the sound file.

## About soundcloud
Soundcloud is a service that allows you to upload and share your music. It is a great place to store your music, but it is also a great place to store your data. The data is stored in the sound file, so you can store as much data as you want. The only limit is the size of the sound file which is 4GB.

## benchmarks
The following benchmarks were made on a 4.600GHz 12th Gen Intel i7-12700H CPU with 16GB of RAM.
### Encoding
| File size | Audio file size | Audio length | Speed | Link |
|-----------|-----------------|------|-------| ---- |
| 2687.94MB | 2687.94MB | 01:28:13 | 323.00s | [Soundcloud-link](https://soundcloud.com/awiteb/pop-os-2204-amd64-intel-23iso) |
| 35.3MB | 35.3MB | 00:01::27 | 4.11s | [Soundcloud-link](https://soundcloud.com/awiteb/rust-1671zip) |
## Decoding
| File size | Audio file size | Audio length | Speed | Link |
|-----------|-----------------|------|-------| ---- |
| 2687.94MB | 2687.94MB | 01:28:13 | ~1930.29s | [Soundcloud-link](https://soundcloud.com/awiteb/pop-os-2204-amd64-intel-23iso) |
| 35.3MB | 35.3MB | 00:01::27 | 25.35s | [Soundcloud-link](https://soundcloud.com/awiteb/rust-1671zip) |


## License
This project is licensed under the MIT license. See the LICENSE file for more information.
