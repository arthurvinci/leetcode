struct NumNode {
    sum: i32,
    left_node: Option<Box<NumNode>>,
    right_node: Option<Box<NumNode>>,
    left_index: usize,
    right_index: usize,
}

impl NumNode {
    fn new(nums: &[i32], l: usize, r: usize) -> Self {
        if l == r {
            Self {
                sum: nums[l],
                left_node: None,
                right_node: None,
                left_index: l,
                right_index: r,
            }
        } else {
            let mid = (l + r) / 2;
            let left_node = Self::new(nums, l, mid);
            let right_node = Self::new(nums, mid + 1, r);

            Self {
                sum: left_node.sum + right_node.sum,
                left_node: Some(Box::new(left_node)),
                right_node: Some(Box::new(right_node)),
                left_index: l,
                right_index: r,
            }
        }
    }

    fn update(&mut self, index: usize, value: i32) {
        if self.left_index == self.right_index && self.right_index == index {
            self.sum = value;
        } else {
            let mid = (self.left_index + self.right_index) / 2;
            if index > mid {
                self.right_node.as_mut().unwrap().update(index, value);
            } else {
                self.left_node.as_mut().unwrap().update(index, value);
            }
            self.sum = self.right_node.as_ref().unwrap().sum + self.left_node.as_ref().unwrap().sum;
        }
    }

    fn sum_range(&self, left: usize, right: usize) -> i32 {
        if self.left_index == left && self.right_index == right {
            self.sum
        } else {
            let mid = (self.left_index + self.right_index) / 2;
            if left > mid {
                self.right_node.as_ref().unwrap().sum_range(left, right)
            } else if right <= mid {
                self.left_node.as_ref().unwrap().sum_range(left, right)
            } else {
                self.right_node.as_ref().unwrap().sum_range(mid + 1, right)
                    + self.left_node.as_ref().unwrap().sum_range(left, mid)
            }
        }
    }
}

struct NumArray {
    root: NumNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            root: NumNode::new(&nums, 0, nums.len() - 1),
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.root.update(index as usize, val)
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.root.sum_range(left as usize, right as usize)
    }
}
