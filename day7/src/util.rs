pub fn permutations(v: Vec<char>) -> PermutationIterator {
    PermutationIterator::new(v)
}

pub struct PermutationIterator {
    i: usize,
    v: Vec<char>,
    child: Option<Box<PermutationIterator>>,
}

impl PermutationIterator {
    fn new(cs: Vec<char>) -> Self {
        PermutationIterator {
            i: 0,
            v: cs,
            child: None,
        }
    }
}

impl Iterator for PermutationIterator {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.v.len() {
            return None;
        }

        if self.v.len() == 1 {
            self.i = 1;
            return Some(self.v.clone());
        }

        if self.child.is_none() {
            let mut spliced_v = self.v.clone();
            spliced_v.remove(self.i);
            self.child = Some(Box::new(PermutationIterator::new(spliced_v)));
        }

        let child_next = self.child.as_mut().unwrap().next();

        match child_next {
            None => {
                self.i += 1;
                self.child = None;
                self.next()
            }
            Some(rest) => {
                let mut rest = rest;
                rest.insert(0, self.v[self.i]);
                Some(rest)
            }
        }
    }
}
