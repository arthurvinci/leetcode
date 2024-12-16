use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        Self::build_adjacency_list(&mut adjacency_list, &prerequisites);
        let mut visited = HashSet::new();
        let mut topological_sort = vec![];

        for i in 0..num_courses {
            if !visited.contains(&i) {
                let current_path = HashSet::new();
                if !Self::dfs(
                    i,
                    &adjacency_list,
                    &mut visited,
                    current_path,
                    &mut topological_sort,
                ) {
                    return vec![];
                }
            }
        }

        topological_sort
    }

    fn build_adjacency_list(
        adjacency_list: &mut HashMap<i32, Vec<i32>>,
        prerequisites: &Vec<Vec<i32>>,
    ) {
        for prerequisite in prerequisites {
            match adjacency_list.get_mut(&prerequisite[0]) {
                None => {
                    adjacency_list.insert(prerequisite[0], vec![prerequisite[1]]);
                }
                Some(neighbours) => neighbours.push(prerequisite[1]),
            }
        }
    }

    fn dfs(
        node: i32,
        adjacency_list: &HashMap<i32, Vec<i32>>,
        visited: &mut HashSet<i32>,
        mut current_path: HashSet<i32>,
        topological_sort: &mut Vec<i32>,
    ) -> bool {
        if !current_path.insert(node) {
            return false;
        }

        if !visited.insert(node) {
            return true;
        }

        for neighbour in adjacency_list.get(&node).unwrap_or(&vec![]) {
            if !Self::dfs(
                *neighbour,
                adjacency_list,
                visited,
                current_path.clone(),
                topological_sort,
            ) {
                return false;
            }
        }
        topological_sort.push(node);
        true
    }
}
