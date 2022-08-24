# Game Of Life

Terminal based Game Of Life implementation with custom input and configurable evolution speed.

## Run

```shell
cargo run -- --help

USAGE:
    learn-rust-game-of-life [OPTIONS]

OPTIONS:
    -c, --custom <NUMBER>         Custom basic configuration
    -h, --help                    Print help information
    -r, --random <NUMBER>         Random basic configuration
        --speed <MILLISECONDS>    Generation speed in milliseconds
```
Random configuration flag has higher priority than custom one.

## TODO

* colorful output
* hide `Space.field` and use [Index](https://doc.rust-lang.org/std/ops/trait.Index.html) and IndexMut traits
