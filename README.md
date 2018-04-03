# shorty

[![Build Status](https://travis-ci.org/keveri/shorty.svg?branch=master)](https://travis-ci.org/keveri/shorty)


Rusty URL Shortener

## Setup

`nightly` is required for rocket library's features
```sh
rustup update
rustup override set nightly
```

## Build & Run

Just build:
```sh
cargo build
```

Build and run:
```sh
cargo run
```

Run tests:
```sh
cargo test
```

## Try it out

Create new short url ID:
```sh
$ curl --data "url=https://www.rust-lang.org" http://localhost:8000/
egYb
```

Lookup a short ID:
```sh
$ curl -I http://localhost:8000/egYb
HTTP/1.1 308 Permanent Redirect
Location: https://www.rust-lang.org
Server: Rocket
Content-Length: 0
Date: Tue, 03 Apr 2018 21:15:35 GMT
```
