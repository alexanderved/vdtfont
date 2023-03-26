use std::{iter, ops, vec};

pub type ElementIdx = usize;

pub struct List<T> {
    pub(crate) data: Vec<Option<T>>,
    free: Vec<ElementIdx>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            free: vec![],
        }
    }

    pub fn add(&mut self, value: T) {
        match self.free.pop() {
            Some(idx) => self.data[idx] = Some(value),
            None => self.data.push(Some(value)),
        }
    }

    pub fn remove(&mut self, index: ElementIdx) -> Option<T> {
        self.data[index].take().map(|element| {
            self.free.push(index);

            element
        })
    }
}

impl<T> iter::FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: iter
                .into_iter()
                .map(|e| Some(e))
                .collect::<Vec<Option<T>>>(),
            free: vec![],
        }
    }
}

impl<T> ops::Index<usize> for List<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> ops::IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
