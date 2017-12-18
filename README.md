## httpbin-rust

This is a simple tool to test the [httpbin](https://httpbin.org) endpoints.

```
$ cargo run -- --help

httpbin test tool 0.1.0
Florian Keller <github@floriankeller.de>
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
