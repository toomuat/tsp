use std::cell::RefCell;

struct UnionFind {
    parent: RefCell<Vec<usize>>,
    size: RefCell<Vec<usize>>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let v = (0..n).collect::<Vec<usize>>();
        UnionFind {
            parent: RefCell::new(v),
            size: RefCell::new(vec![1; n]),
        }
    }

    pub fn root(&self, i: usize) -> usize {
        let mut v = self.parent.borrow_mut();
        if v[i] == i {
            return i;
        }
        v[i] = self.root(v[i]);
        v[i] as usize
    }

    /// Parent of a is b
    pub fn add(&self, a: usize, b: usize) {
        let mut v = self.parent.borrow_mut();
        v[a] += b;
    }

    pub fn unite(&self, a: usize, b: usize) {
        let mut parent_vec = self.parent.borrow_mut();
        let mut size_vec = self.size.borrow_mut();
        let a_root = self.root(parent_vec[a]);
        let b_root = self.root(parent_vec[b]);

        if a_root == b_root {
            return;
        }

        if self.size(a) < self.size(b) {
            size_vec[a] += size_vec[b];
            parent_vec[b] = a;
        } else {
            size_vec[b] += size_vec[a];
            parent_vec[a] = b;
        }
    }

    pub fn size(&self, i: usize) -> usize {
        let v = self.size.borrow();
        v[self.root(i)]
    }

    pub fn print(&self) {
        let parent_vec = self.parent.borrow();
        let size_vec = self.size.borrow();

        dbg!(parent_vec);
        dbg!(size_vec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unionfind() {
        let mut uf = UnionFind::new(5);
        uf.print();
        uf.unite(0, 1);
        uf.unite(1, 2);
        (0..5).for_each(|i| println!("Root of {} is {}", i, &uf.root(i)));
    }
}
