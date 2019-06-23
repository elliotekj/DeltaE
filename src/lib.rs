//! # DeltaE
//!
//! DeltaE is a pure-Rust implementation of the [CIEDE2000
//! algorithm](http://en.wikipedia.org/wiki/Color_difference#CIEDE2000) which
//! serves to quantify the difference between two colors.
//!
//! ## Example:
//!
//! ```
//! extern crate delta_e;
//! extern crate lab;
//!
//! use delta_e::DE2000;
//! use lab::Lab;
//!
//! fn main() {
//!     let color_1 = Lab {
//!         l: 38.972,
//!         a: 58.991,
//!         b: 37.138,
//!     };
//!
//!     let color_2 = Lab {
//!         l: 54.528,
//!         a: 42.416,
//!         b: 54.497,
//!     };
//!
//!     let delta_e = DE2000::new(color_1, color_2, Default::default());
//!     println!("The color difference is: {}", delta_e);
//! }
//! ```

extern crate lab;

use std::f32;
use lab::Lab;

mod de2000;

pub use de2000::{ DE2000, KSubArgs };
