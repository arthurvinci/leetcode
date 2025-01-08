use std::cmp::Ordering;

pub struct Solution;

struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        Self {
            roots: (0..n as usize).collect(),
            ranks: vec![0; n as usize],
        }
    }

    fn find(&mut self, n: i32) -> i32 {
        let n = n as usize;

        if self.roots[n] != n {
            self.roots[n] = self.find(self.roots[n] as i32) as usize;
        }

        self.roots[n] as i32
    }

    fn union(&mut self, n1: i32, n2: i32) {
        let root1 = self.find(n1) as usize;
        let root2 = self.find(n2) as usize;

        if root1 != root2 {
            match self.ranks[root1].cmp(&self.ranks[root2]) {
                Ordering::Less => self.roots[root1] = root2,

                Ordering::Equal => {
                    self.ranks[root1] += 1;
                    self.roots[root2] = root1;
                }
                Ordering::Greater => self.roots[root2] = root1,
            }
        }
    }

    fn all_same(&mut self) -> bool {
        let first_parent = self.roots[0] as i32;
        (1..self.roots.len() as i32).all(|i| self.find(i) == first_parent)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    node1: i32,
    node2: i32,
    weight: i32,
}

impl Edge {
    fn from(edge: &[i32]) -> Self {
        Self {
            node1: edge[0],
            node2: edge[1],
            weight: edge[2],
        }
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges: Vec<Edge> = edges.into_iter().map(|edge| Edge::from(&edge)).collect();
        edges.sort_unstable_by_key(|edge| edge.weight);

        let real_weight = Self::kruskal(n, &edges, None, None);
        let mut criticals = vec![];
        let mut pseudo_criticals = vec![];

        for i in 0..edges.len() {
            if Self::kruskal(n, &edges, Some(edges[i]), None) != real_weight {
                criticals.push(i as i32)
            } else if Self::kruskal(n, &edges, None, Some(edges[i])) == real_weight {
                pseudo_criticals.push(i as i32)
            }
        }

        vec![criticals, pseudo_criticals]
    }

    fn kruskal(n: i32, edges: &[Edge], ignore: Option<Edge>, forced: Option<Edge>) -> i32 {
        let mut total_weight = 0;
        let mut union_find = UnionFind::new(n);

        if let Some(edge) = forced {
            total_weight += edge.weight;
            union_find.union(edge.node1, edge.node2);
        }

        for edge in edges {
            if Some(*edge) != ignore && union_find.find(edge.node1) != union_find.find(edge.node2) {
                total_weight += edge.weight;
                union_find.union(edge.node1, edge.node2);
            }
        }

        if union_find.all_same() {
            total_weight
        } else {
            i32::MAX
        }
    }
}
