# Malory
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

Malory is a rust async benchmarking tool using Tokio.

[crates-badge]: https://img.shields.io/crates/v/malory.svg
[crates-url]: https://crates.io/crates/malory
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/dt665m/malory-rs/blob/master/LICENSE
[actions-badge]: https://github.com/dt665m/malory-rs/workflows/CI/badge.svg
[actions-url]: https://github.com/dt665m/malory-rs/actions?query=workflow%3ACI+branch%3Amaster

## Usage

```
use malory;

#[tokio::main]
fn main() {
  let iterations = 1000;
  let parallelism = 5;
  let context = "That's how you get ants.";

  malory::judge_me(iterations, parallelism, context, |ctx, i| async { true }).await;
}
```
