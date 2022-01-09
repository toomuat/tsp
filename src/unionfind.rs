pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.root(self.parent[i]);
        self.parent[i]
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let mut a_root = self.root(self.parent[a]);
        let mut b_root = self.root(self.parent[b]);

        if a_root == b_root {
            return;
        }

        if self.size[a_root] < self.size[b_root] {
            std::mem::swap(&mut a_root, &mut b_root);
        }

        self.size[a_root] += self.size[b_root];
        self.parent[b_root] = a_root;
    }

    pub fn size(&mut self, i: usize) -> usize {
        let parent_idx = self.root(i);
        self.size[parent_idx]
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        let a_root = self.root(self.parent[a]);
        let b_root = self.root(self.parent[b]);
        a_root == b_root
    }

    pub fn print(&self) {
        dbg!(&self.parent);
        dbg!(&self.size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unionfind() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 2);
        uf.unite(2, 3);
        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(1), 1);
        assert_eq!(uf.root(2), 0);
        assert_eq!(uf.root(3), 0);
        assert_eq!(uf.root(4), 4);
        assert_eq!(uf.size(3), 3);
    }
}
