use crate::{Coords, Space};

#[test]
fn peek_and_poke() {
    let mut s = Space::new_empty(Coords::new(5, 3));
    for x in 0..s.width() {
        for y in 0..s.height() {
            s[Coords::new(x, y)] = x ^ 2 + y ^ 2;
        }
    }

    for x in 0..s.width() {
        for y in 0..s.height() {
            assert_eq!(s[Coords::new(x, y)], x ^ 2 + y ^ 2);
        }
    }
}
