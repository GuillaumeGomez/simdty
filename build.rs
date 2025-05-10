use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn simd_type<W: Write>(w: &mut W, t: &str, width: u32, length: u32) {
    assert!(length >= 2);
    assert!(t == "f" || t == "u" || t == "i");

    let ty = format!("{}{}", t, width);
    writeln!(w, "\
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
/// {length} values of type {ty} in a single SIMD vector.
pub struct {ty}x{length}(pub [{ty}; {length}]);").unwrap()
}

fn main() {
    let path = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&path);
    let mut out = File::create(&dst.join("types.rs")).unwrap();
    for length in [2, 4, 8, 16, 32, 64].iter().cloned() {
        for &int in ["i", "u"].iter() {
            for &int_width in [8, 16, 32, 64].iter() {
                simd_type(&mut out, int, int_width, length)
            }
        }

        let float = "f";
        for &float_width in [32, 64].iter() {
            simd_type(&mut out, float, float_width, length)
        }
    }
}
