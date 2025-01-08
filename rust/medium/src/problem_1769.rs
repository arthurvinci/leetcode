struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut to_move_right = vec![(0, 0); boxes.len()];
        let mut to_move_left = vec![(0, 0); boxes.len()];

        let boxes_chars: Vec<char> = boxes.chars().collect();

        if boxes_chars[0] == '1' {
            to_move_right[0] = (0, 1);
        }

        for index in 1..boxes.len() {
            let prev = to_move_right[index - 1];
            to_move_right[index] = prev;
            to_move_right[index].0 += prev.1;

            if boxes_chars[index] == '1' {
                to_move_right[index].1 += 1;
            }
        }

        if boxes_chars[boxes.len() - 1] == '1' {
            to_move_left[boxes.len() - 1] = (0, 1)
        }

        for index in (0..boxes.len() - 1).rev() {
            let next = to_move_left[index + 1];
            to_move_left[index] = next;
            to_move_left[index].0 += next.1;

            if boxes_chars[index] == '1' {
                to_move_left[index].1 += 1;
            }
        }

        let mut result = vec![0; boxes.len()];

        for i in 0..boxes.len() {
            result[i] = to_move_left[i].0 + to_move_right[i].0;
        }

        result
    }
}
