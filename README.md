# `ipecho` utility

Another quick utility in Rust to try out making HTTP calls. I use the [ipecho
service] often in scripts to get my external IPv4 address. The service's policy
on rate limiting is just that users should cache the result for a "reasonable
period of time". This utility uses a cache file to only check every 15 minutes.

[ipecho service]: http://ipecho.net

## Building and installing

Install Rust and clone this repository. Inside this directory, type `cargo
build --release` and the binary will be in `./target/release/`.
