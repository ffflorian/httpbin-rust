# httpbin-rust [![Build Status](https://action-badges.now.sh/ffflorian/httpbin-rust)](https://github.com/ffflorian/httpbin-rust/actions/)

This is a simple tool to test the [httpbin](https://httpbin.org) endpoints.

## Working features

* Basic HTTP methods (GET, POST, DELETE, ...)
* String data transfer
* Custom path (e.g. <a href="http://httpbin.org/ip">http://httpbin.org<strong>/ip</strong></a>)

## Missing features

* Basic Auth
* Brotli (sending / receiving)
* Deflate (sending / receiving)
* Gzip (sending / receiving)
* Images (sending / receiving)

## Usage

```
$ cargo run -- --help

httpbin test tool 0.1.0
Florian Imdahl <git@ffflorian.de>
test httpbin endpoints

USAGE:
    httpbin [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --data <data>        Sets the data to transfer [default: ]
    -m, --method <method>    Sets the method to use [default: GET]
    -p, --path <path>        Sets a custom path [default: /ip]
```
