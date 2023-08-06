# Mosquito 🪰 [![crates.io](https://img.shields.io/crates/v/mosquito.svg)](https://crates.io/crates/mosquito)


Mosquito is a simple HTTP server that prints the parameters of incoming requests to the console in a beautiful manner.

## Introduction
Mosquito was born out of the necessity to ensure the accuracy of requests in various contexts, particularly within the realm of mobile development and gaming platforms. As applications grow more complex, the ability to monitor and validate the correctness of requests becomes paramount. Whether working with React Native or building analytics tools for gaming, Mosquito serves as a reliable tool to intercept and log HTTP requests. By visualizing these requests, developers can gain insights, diagnose issues, and confirm that everything is functioning as intended, enhancing the overall quality and reliability of the applications.


## Requirements

- Rust Programming Language
- Cargo package manager

The following Rust crates are used in this project:

- `hyper`: For creating the HTTP server.
- `clap`: For handling command-line arguments.
- `serde_json`: For JSON processing.

## Installation

### Using Cargo

You can install Mosquito directly from crates.io by running the following command:

**`cargo install mosquito`**

### Downloading from GitHub Releases
Alternatively, you can download the compiled binary from the [Releases](https://github.com/kubilaysalih/mosquito/releases) section of the GitHub repository.

### Building from Source

Clone the repository and build the project using Cargo:

**`cargo build --release`**

The compiled binary will be available in the `target/release` directory.

## Usage

### Using Cargo

To start the server, run the following command in the project directory:

```shell
cargo run -- --host {HOST_IP} --port {PORT}
```
### Using the Compiled Binary

After compiling the project, you can run the `mosquito` binary directly. Replace the path with the location of the compiled binary:

```shell
mosquito --host {HOST_IP} --port {PORT}
```
- `{HOST_IP}`: The IP address the server will connect to. Optional, defaults to the local IP address.
- `{PORT}`: The port the server will listen on. Optional, defaults to 80.

Example:

```shell
mosquito --host 0.0.0.0 --port 8080
```
These commands start the server at `http://0.0.0.0:8080`.

## Logs

The server prints the timestamp, method, URI, and content of each request to the console. If the content is in JSON format, it is printed in a readable manner.

## Contributing

This project is open-source and awaits your contributions. Feel free to make changes and send a pull request.

## License

This project is licensed under the MIT license. For more details, see the `LICENSE` file.

---

[![README Created with ChatGPT](https://img.shields.io/badge/README%20Created%20with-ChatGPT-blue.svg)](https://openai.com)
