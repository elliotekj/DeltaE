# DeltaE - Quantify color differences in Rust

[![Crates.io](https://meritbadge.herokuapp.com/delta_e)](https://crates.io/crates/delta_e)
[![Docs](https://docs.rs/delta_e/badge.svg)](https://docs.rs/delta_e)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/elliotekj/DeltaE/blob/master/LICENSE)

DeltaE is a pure-Rust implementation of the [CIEDE2000
algorithm](http://en.wikipedia.org/wiki/Color_difference#CIEDE2000) which serves
to quantify the difference between two colors. It is entirely based on the work
of [Zachary Schuessler](http://zaclee.net/), who has written a [Javascript
implementation](https://github.com/zschuessler/DeltaE/blob/master/src/dE00.js)
of the algorithm.

## Installation

If you're using Cargo, just add DeltaE to your `Cargo.toml`:

```toml
[dependencies]
delta_e = "0.2"
```

## Example

```rust
extern crate delta_e;
extern crate lab;

use delta_e::DE2000;
use lab::Lab;

fn main() {
    let color_1 = Lab {
        l: 38.972,
        a: 58.991,
        b: 37.138,
    };

    let color_2 = Lab {
        l: 54.528,
        a: 42.416,
        b: 54.497,
    };

    let delta_e = DE2000::new(color_1, color_2, Default::default());
    println!("The color difference is: {}", delta_e);
}
```

## License

DeltaE is released under the MIT [`LICENSE`](/elliotekj/DeltaE/blob/master/LICENSE).

## About

This crate was written by [Elliot Jackson](https://elliotekj.com).

- Blog: [https://elliotekj.com](https://elliotekj.com)
- Email: elliot@elliotekj.com
