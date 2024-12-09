use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ranks = vec![0; edges.len() + 1];
        let mut parents: Vec<usize> = (0..=edges.len() + 1).into_iter().collect();
        let mut last_false_edge = vec![];

        for edge in edges {
            if !Self::union(&mut parents, &mut ranks, edge[0] as usize, edge[1] as usize) {
                last_false_edge = edge;
            }
        }

        last_false_edge
    }

    fn find(parents: &mut [usize], mut node: usize) -> usize {
        let mut parent = parents[node];
        while parent != node {
            parents[node] = parents[parent];
            node = parent;
            parent = parents[parent];
        }
        parent
    }

    fn union(parents: &mut [usize], ranks: &mut [usize], node_1: usize, node_2: usize) -> bool {
        let parent_1 = Self::find(parents, node_1);
        let parent_2 = Self::find(parents, node_2);

        if parent_1 == parent_2 {
            false
        } else {
            match ranks[parent_1].cmp(&ranks[parent_2]) {
                Ordering::Less => {
                    parents[parent_1] = parent_2;
                }
                Ordering::Equal => {
                    parents[parent_2] = parent_1;
                    ranks[parent_2] += 1;
                }
                Ordering::Greater => {
                    parents[parent_2] = parent_1;
                }
            }
            true
        }
    }
}
