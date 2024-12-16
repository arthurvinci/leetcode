use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        Self::build_adjacency_list(&mut adjacency_list, &prerequisites);
        let mut visited = HashSet::new();

        for i in 0..num_courses {
            if !visited.contains(&i) {
                let current_path = HashSet::new();
                if !Self::dfs(i, &adjacency_list, &mut visited, current_path) {
                    return false;
                }
            }
        }

        true
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
    ) -> bool {
        if !current_path.insert(node) {
            return false;
        }

        if !visited.insert(node) {
            return true;
        }

        for neighbour in adjacency_list.get(&node).unwrap_or(&vec![]) {
            if !Self::dfs(*neighbour, adjacency_list, visited, current_path.clone()) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0207::Solution;

    #[test]
    fn test_1() {
        assert!(!Solution::can_finish(2, vec![vec![0, 1], vec![1, 0]]))
    }

    #[test]
    fn test_2() {
        assert!(Solution::can_finish(
            3,
            vec![vec![0, 1], vec![0, 2], vec![1, 2]]
        ))
    }
}
