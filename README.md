# hyperid-rs [![Rust](https://github.com/allevo/hyperid-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/allevo/hyperid-rs/actions/workflows/rust.yml)
Superfast id generator

This is a porting of hyper written in [javascript](https://github.com/mcollina/hyperid). Thanks to [mcollina](https://github.com/mcollina) for that work

## Install

Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
hyperid = "1"
```

## Usage

```rust

use hyperid::HyperId;

fn main() {
    let mut hyperid = HyperId::default();

    let id1 = hyperid.generate();
    let id2 = hyperid.generate();

    assert_ne!(id1, id2);

    println!("{}", id1.to_url_safe()); // prints "3ZAYYJilG7vHTqiUuaQdFg.0"
}
```

## Benchmark
This crate borns for providing a fast id generator.
Comparing with uuid crates, on my computer:

```
$ cargo bench
...
test hyperid ... bench:           9 ns/iter (+/- 1)
test uuid    ... bench:       1,657 ns/iter (+/- 148)
...
```


## Prepare for release

```sh
cargo release  -vv --skip-publish <version>
# wait for CI
git checkout <tag>
cargo publish
```


## License

See [LICENSE](./LICENSE) file