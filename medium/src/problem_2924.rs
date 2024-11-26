struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut has_in_degree = vec![true; n as usize];
        for edge in edges {
            has_in_degree[edge[1] as usize] = false;
        }

        let mut champion = None;

        for (vertex, is_champion) in has_in_degree.iter().enumerate(){
            if *is_champion{
                match champion {
                    None => {
                        champion = Some(vertex as i32)
                    }
                    Some(_) => {
                        return -1
                    }
                }
            }
        }

        champion.unwrap()
    }
}