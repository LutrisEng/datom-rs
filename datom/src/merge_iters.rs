use std::mem;

// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

pub enum OriginIter {
    A,
    B,
}

pub struct MergeIters<T: Ord, A: Iterator<Item = T>, B: Iterator<Item = T>> {
    a: A,
    b: B,
    a_front: Option<Option<T>>,
    a_back: Option<Option<T>>,
    b_front: Option<Option<T>>,
    b_back: Option<Option<T>>,
}

impl<T: Ord, A: Iterator<Item = T>, B: Iterator<Item = T>> Iterator for MergeIters<T, A, B> {
    type Item = (T, OriginIter);

    fn next(&mut self) -> Option<Self::Item> {
        let a = if let Some(a_val) = mem::replace(&mut self.a_front, None) {
            a_val
        } else {
            self.a.next()
        };
        let b = if let Some(b_val) = mem::replace(&mut self.b_front, None) {
            b_val
        } else {
            self.b.next()
        };
        let (res, a, b) = if let Some(a) = a {
            if let Some(b) = b {
                if a < b {
                    (Some((a, OriginIter::A)), self.a.next(), Some(b))
                } else {
                    (Some((b, OriginIter::B)), Some(a), self.b.next())
                }
            } else {
                (Some((a, OriginIter::A)), self.a.next(), None)
            }
        } else if let Some(b) = b {
            (Some((b, OriginIter::B)), None, self.b.next())
        } else {
            (None, None, None)
        };
        self.a_front = Some(a);
        self.b_front = Some(b);
        res
    }
}

impl<
        T: Ord,
        A: Iterator<Item = T> + DoubleEndedIterator,
        B: Iterator<Item = T> + DoubleEndedIterator,
    > DoubleEndedIterator for MergeIters<T, A, B>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let a = if let Some(a_val) = mem::replace(&mut self.a_back, None) {
            a_val
        } else {
            self.a.next_back()
        };
        let b = if let Some(b_val) = mem::replace(&mut self.b_back, None) {
            b_val
        } else {
            self.b.next_back()
        };
        let (res, a, b) = if let Some(a) = a {
            if let Some(b) = b {
                if a > b {
                    (Some((a, OriginIter::A)), self.a.next_back(), Some(b))
                } else {
                    (Some((b, OriginIter::B)), Some(a), self.b.next_back())
                }
            } else {
                (Some((a, OriginIter::A)), self.a.next_back(), None)
            }
        } else if let Some(b) = b {
            (Some((b, OriginIter::B)), None, self.b.next_back())
        } else {
            (None, None, None)
        };
        self.a_back = Some(a);
        self.b_back = Some(b);
        res
    }
}

impl<T: Ord, A: Iterator<Item = T>, B: Iterator<Item = T>> MergeIters<T, A, B> {
    /// Merge n sorted iterators
    pub const fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            a_front: None,
            a_back: None,
            b_front: None,
            b_back: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MergeIters;

    #[test]
    fn basic_test() {
        let first: [u64; 4] = [1, 76, 2754, 53326];
        let second: [u64; 3] = [53, 62, 431];
        let merged = MergeIters::new(first.iter().cloned(), second.iter().cloned());
        let results: Vec<u64> = merged.map(|x| x.0).collect();
        assert_eq!(vec![1, 53, 62, 76, 431, 2754, 53326], results);
    }

    #[test]
    fn basic_reverse_test() {
        let first: [u64; 4] = [1, 76, 2754, 53326];
        let second: [u64; 3] = [53, 62, 431];
        let merged = MergeIters::new(first.iter().cloned(), second.iter().cloned());
        let results: Vec<u64> = merged.rev().map(|x| x.0).collect();
        assert_eq!(vec![53326, 2754, 431, 76, 62, 53, 1], results);
    }
}
