# IP Info Fetcher

Rust program that fetches and displays your IP address along with its geographical location using the `ipinfo.io` API.

## Dependencies

This project uses the following dependencies:

- `reqwest`: For making HTTP requests
- `serde`: For parsing JSON responses
- `tokio`: For asynchronous runtime

## Installation

To build and run this project, you need to have Rust and Cargo installed on your system. 

### Instalation on linux
```
cargo build --release
sudo cp target/release/ip_info /usr/local/bin/
```

### Usage

Run terminal 
```
ip_info
```
