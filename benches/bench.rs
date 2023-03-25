#![allow(unused_variables)]
#![feature(test)]

// fn main() {
extern crate test;
extern crate conway;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = conway::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
// }