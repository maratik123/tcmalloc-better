# TCMalloc better
A Rust wrapper over Google's TCMalloc memory allocator

[![Latest Version]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [TCMalloc] allocator.
TCMalloc is a general-purpose, performance-oriented allocator built by Google.

## Comparison with other tcmalloc wrappers
Current TCMalloc wrappers rely on [gperftools](https://github.com/gperftools/gperftools), which has been in a detached state from the main development branch
 for over 10 years and lacks modern features such as per-CPU caching.

* [tcmalloc](https://crates.io/crates/tcmalloc) - Cons:
  * Outdated wrapper, which does not updates for years
  * Depends on gperftools-2.7
* [tcmalloc2](https://crates.io/crates/tcmalloc2) - Cons:
  * Wrapper which can not build in offline mode
  * Depends on gperftools-2.16

## Caveats

This library is intended for use on Linux (x86_64, aarch64).
For applications requiring support on a broader range of platforms,
consider using [malloc-best-effort], which automatically selects
the best allocator based on the target platform.

## Usage

```rust
use tcmalloc_better::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

fn main() {
    TCMalloc::process_background_actions_thread();

    // Rest of main
}
```

## Requirements

A __C++__ compiler is required for building [TCMalloc] with cargo.

## Preparing native sources

This workspace vendors Google TCMalloc and Abseil inside `libtcmalloc-sys/c_src`.
Run `git submodule update --init --recursive` after cloning to populate those directories before building.

## See also
* [malloc-best-effort] - library, which automatically selects
                         the best suited allocator based on the target platform

[crates.io]: https://crates.io/crates/tcmalloc-better
[Latest Version]: https://img.shields.io/crates/v/tcmalloc-better.svg
[Documentation]: https://docs.rs/tcmalloc-better/badge.svg
[docs.rs]: https://docs.rs/tcmalloc-better
[TCMalloc]: https://github.com/google/tcmalloc
[malloc-best-effort]: https://crates.io/crates/malloc-best-effort
