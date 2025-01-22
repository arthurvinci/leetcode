struct StockSpanner {
    stack: Vec<(i32, usize)>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut total_span = 1;
        loop {
            let top = self.stack.pop();
            if let Some((prev_price, span)) = top {
                if prev_price <= price {
                    total_span += span;
                } else {
                    self.stack.push((prev_price, span));
                    break;
                }
            } else {
                break;
            }
        }
        self.stack.push((price, total_span));

        total_span as i32
    }
}
