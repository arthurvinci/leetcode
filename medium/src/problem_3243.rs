use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![false; n as usize]; n as usize];
        let mut distances = vec![];
        for i in 0..n as usize {
            if i + 1 < n as usize{
                graph[i][i +1] = true;
            }
        }

        for query in queries{
            graph[query[0] as usize][query[1] as usize] = true;
            distances.push(Self::dijkstra(n as usize, &graph));
        }

        distances
    }

    fn dijkstra(n: usize, graph: &[Vec<bool>]) -> i32 {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(0);

        let mut depth = 0;
        while !queue.is_empty(){
            let element_amount = queue.len();
            for _ in 0..element_amount{
                let elem = queue.pop_front().unwrap();

                if elem == n-1 {
                    return depth
                }

                if seen.insert(elem){
                    for i in 0..n{
                        if graph[elem][i] {
                            queue.push_back(i);
                        }
                    }
                }
            }

            depth += 1;
        }
        -1
    }
}