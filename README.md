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

Build:
```sh
cargo build
```

Run tests:
```sh
cargo test
```

Run with docker:
```sh
docker-compose up
```

Run on host:
```sh
# install and start redis locally
REDIS_HOST=localhost cargo run
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
