# Algorithms in Rust

- Multiples bins
- Combined resulted
- Valgrind resulted memory
- Configs to build optimized

## Requirements to run
- [Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## How run?
```
cargo run --bin insertion_sort --release
```
> All bins in `src/bin` folder
> To minimal file size (without dependencies rust in bin), use strip=symbols in command

## Generating report from Valgrind
```
valgrind ./program > log.txt 2>&1
```

## Reports folders
- `results_time`
- `results_valgrind`

## Utils

- [Reduce size final release](https://www.collabora.com/news-and-blog/blog/2020/04/28/reducing-size-rust-gstreamer-plugin/)
- [Prepare to benchmark](https://easyperf.net/blog/2019/08/02/Perf-measurement-environment-on-Linux)
