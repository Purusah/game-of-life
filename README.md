# Game Of Life

Terminal based Game Of Life implementation with custom input and configurable evolution speed.

## Run

```shell
cargo run

USAGE:
    learn-rust-game-of-life [OPTIONS]

OPTIONS:
    -c, --custom                  Custom basic configuration
    -h, --help                    Print help information
    -r, --random                  Random basic configuration
        --speed <MILLISECONDS>    Generation speed in milliseconds
```
Random configuration flag has higher priority than custom one.

## TODO

* Custom input configurable space size
* colorful output
* nicer cells
* `Space` with `impl`
