# skeever

[![build api](https://github.com/corvidaelabs/skeever/actions/workflows/build-api.yml/badge.svg)](https://github.com/corvidaelabs/skeever/actions/workflows/build-api.yml)
[![build web](https://github.com/corvidaelabs/skeever/actions/workflows/build-web.yml/badge.svg)](https://github.com/corvidaelabs/skeever/actions/workflows/build-web.yml)

Skeever is an in-world Oblivion social network

## Development

### Requirements

- NATS JetStream
- Node
- Rust

### High Level Architecture

The skeever API is responsible for connecting and streaming events from the
oddlaws event stream into the frontend. It provides the websocket server

The frontend is responsible for displaying and interacting with the skeever API (mostly just displaying the squeaks atm)
