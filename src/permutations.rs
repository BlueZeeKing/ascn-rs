pub struct Permutation<'a, T> {
    a: &'a [T],
    b: &'a [T],
    a_pos: usize,
    b_pos: usize,
}

impl<'a, T> Permutation<'a, T> {
    pub fn new(a: &'a [T], b: &'a [T]) -> Self {
        Self {
            a,
            b,
            a_pos: 0,
            b_pos: 0,
        }
    }
}

impl<'a, T> Iterator for Permutation<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.b_pos == self.b.len() {
            self.a_pos += 1;
            self.b_pos = 0;
        }

        if self.a_pos >= self.a.len() {
            return None;
        }

        self.b_pos += 1;

        return Some((&self.a[self.a_pos], &self.b[self.b_pos - 1]));
    }
}

#[cfg(test)]
mod tests {
    use super::Permutation;

    #[test]
    fn test() {
        let permutation = Permutation::new(&[1, 2], &[3, 4]).collect::<Vec<_>>();

        assert_eq!(permutation, vec![(&1, &3), (&1, &4), (&2, &3), (&2, &4)])
    }
}