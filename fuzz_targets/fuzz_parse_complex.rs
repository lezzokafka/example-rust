#[macro_use] 
extern crate afl;
extern crate example_rust;

fn main() {
    fuzz!(|data: &[u8]| {
        let _ = example_rust::parse_complex(data);
    });
}