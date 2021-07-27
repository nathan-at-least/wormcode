#[cfg(test)]
mod tests;

use crate::Coords;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Space<T> {
    bottomright: Coords,
    v: Vec<T>,
}

impl<T> Space<T> {
    pub fn width(&self) -> u32 {
        self.bottomright.x
    }

    pub fn height(&self) -> u32 {
        self.bottomright.y
    }

    pub fn map_cells<F, U>(mut self, f: F) -> Space<U>
    where
        F: FnMut(T) -> U,
    {
        Space {
            bottomright: self.bottomright,
            v: self.v.drain(..).map(f).collect(),
        }
    }

    fn coords_ix(&self, c: Coords) -> usize {
        use std::convert::TryInto;

        (self.width() * c.y + c.x).try_into().unwrap()
    }
}

impl<T> Space<T>
where
    T: Default,
{
    pub fn new_empty(bottomright: Coords) -> Space<T> {
        use std::convert::TryInto;

        let len = bottomright.x * bottomright.y;
        let mut v = Vec::with_capacity(len.try_into().unwrap());
        for _ in 0..len {
            v.push(T::default());
        }

        Space { bottomright, v }
    }
}

impl<T> Index<Coords> for Space<T> {
    type Output = T;

    fn index(&self, index: Coords) -> &T {
        let ix = self.coords_ix(index);
        &self.v[ix]
    }
}

impl<T> IndexMut<Coords> for Space<T> {
    fn index_mut(&mut self, index: Coords) -> &mut T {
        let ix = self.coords_ix(index);
        &mut self.v[ix]
    }
}
