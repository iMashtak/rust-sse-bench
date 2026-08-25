# Rust SSE Benchmark

Server-Sent Events protocol impls comparison. No correctness checks, just performance.

Analyzed crates:

- [`reqwest-eventsource`](https://github.com/jpopesculian/reqwest-eventsource)
- [`eventsource-client`](https://github.com/launchdarkly/rust-eventsource-client)
- [`reqwest-sse`](https://github.com/vvvinceocam/reqwest-sse)
- [`sse-reqwest-client`](https://github.com/PizzasBear/sse-rs)
- [`sse-stream`](https://github.com/4t145/sse-stream/)
- [`sseer`](https://github.com/SneedSeedFeed/sseer)
- [`eventsource-stream2`](https://github.com/SneedSeedFeed/eventsource-stream)

## Setup

Because many crates strictly depends on `reqwest`, benchmark requires SSE-stub running on `localhost:8080`. Benchmark consumes 100 events from stub in one iteration.

Each test differs from each other by content of message. There are 4 modes:

- [`single.txt`](./src/single.txt) - message containing single letter. Needed for baseline.
- [`long-line.txt`](./src/long-line.txt) - message containing single long line of symbols.
- [`many-lines.txt`](./src/many-lines.txt) - message containing many lines of the same symbol.
- [`gigantic.txt`](./src/gigantic.txt) - message with many lines of relatively long lines.

## Single (baseline)

![](./resources/single-violin.svg)

All cases perform at the same duration as expected. 

## Long Line

![](./resources/long-line-violin.svg)

On a long line payload there is no significant difference between analyzed crates.

## Many Lines

![](./resources/many-lines-violin.svg)

`sse-strean` is the fastest.

## Gigantic

![](./resources/gigantic-violin.svg)

`sse-reqwest-client` is the fastest.
