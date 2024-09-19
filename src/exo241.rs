/*
pub enum Node{
    Add,
    Mul,
    Sub,
    Number(i32)
}

impl Node{
    pub fn nodes_from(expression: String) -> Vec<Node>{
        let mut nodes = vec![];
        let mut current_str = "".to_string();

        for char in expression.chars() {
            match char{
                '-' => {
                    nodes.push(Node::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Node::Sub);
                    current_str = "".to_string();
                }

                '+' =>{
                    nodes.push(Node::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Node::Add);
                    current_str = "".to_string();
                }

                '*'=> {
                    nodes.push(Node::Number(str::parse(&current_str).unwrap()));
                    nodes.push(Node::Mul);
                    current_str = "".to_string();
                }
                _ =>{
                    current_str.push(char);
                }
            }
        }

        if current_str.len() > 0 {
            nodes.push(Node::Number(str::parse(&current_str).unwrap()));
        }

        nodes
    }
}

pub struct Tree{
    pub node: Node,
    pub left_tree: Option<Tree>,
    pub right_tree: Option<Tree>
}

impl Tree{

    pub fn build_from(expression: String) -> Self{
        let mut nodes = Node::nodes_from(expression);
        let mut last_node = nodes.pop();

        let mut start_tree = match last_node {
            None => { panic!() }
            Some(node) => {
                Tree{
                    node: node,
                    left_tree: None,
                    right_tree: None
                }
            }
        };

        Self::add_nodes(&mut start_tree, nodes);

        start_tree
    }

    pub fn all_ways_to_add(&self) -> Vec<i32> {

    }

    fn add_nodes(tree: &mut Tree, mut nodes: Vec<Node>) {
        match nodes.pop(){
            None => {}
            Some(node) => {
                let mut new_tree = Tree{
                    node: node,
                    left_tree: None,
                    right_tree: None
                };

                Self::add_nodes(&mut new_tree, nodes);

                tree.right_tree = Some(new_tree);
            }
        }
    }

}


pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    if expression.len() == 0{
        return vec![0]
    }

    if expression.len() == 1 {
        return vec![str::parse::<i32>(&expression).unwrap()];
    }

    let mut sub_expressions: Vec<String> = split(expression);

    let mut result: Vec<i32> = vec![];
    let mut current_eval = 0;
    let mut last_char = "".to_string();

    loop {
        let current_word = sub_expressions.pop();
        match current_word{
            None => {
                return result
            }
            Some(new_word) => {
                match new_word.as_str() {
                    "-" | "+" | "*" => {
                        let other_computes = diff_ways_to_compute(to_string(&sub_expressions));
                        for other in other_computes{
                            result.push(to_op(&new_word, other, current_eval));
                        }
                        last_char = new_word;
                    }
                    _ => {
                        let nb = str::parse::<i32>(&new_word).unwrap();
                        current_eval = to_op(&last_char, nb, current_eval);
                    }
                }
            }
        }
    }
}

fn to_op(op: &str, lval: i32, rval: i32) -> i32 {
    match op {
        "-" => {
            return lval - rval;
        }
        "+" => {
            return lval + rval;
        }
        "*" => {
            return lval * rval;
        }

        "" =>{
            return lval
        }

        _ =>{
            panic!("Not possible")
        }
    }
}
fn to_string(expr: &Vec<String>) -> String{
    let mut new_string = String::new();
    for exp in expr{
        new_string.push_str(exp);
    }

    new_string
}


#[cfg(test)]
mod test {
    use crate::exo241::diff_ways_to_compute;

    #[test]
    fn test_1() {
        let ok = diff_ways_to_compute("2-1-1".to_string());
        assert_eq!(ok, vec![0,2])
    }

    #[test]
    fn test_2() {
        let ok = diff_ways_to_compute("2*3-4*5".to_string());
        assert_eq!(ok, vec![-34,-14,-10,-10,10])
    }
}*/