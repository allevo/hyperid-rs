# hyperid-rs [![Rust](https://github.com/allevo/hyperid-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/allevo/hyperid-rs/actions/workflows/rust.yml)
Superfast id generator

This is a porting of hyper written in [javascript](https://github.com/mcollina/hyperid). Thanks to [mcollina](https://github.com/mcollina) for that work

## Install

## Prepare for release

```
cargo release  -vv --skip-publish <version>
# wait for CI
git checkout <tag>
cargo publish
```


## License

See [LICENSE](./LICENSE) file