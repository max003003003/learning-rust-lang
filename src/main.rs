
struct Node<T> {
    value: T,
    left_node:   Option<Box<Node<T>>>,
    right_node:  Option<Box<Node<T>>>,
}
fn print_tree<T: std::fmt::Display> (tree: Option<Box<Node<T>>> ) {
    match tree {
    Some(x) => {
        print_tree(x.left_node);
        println!("{}", x.value);
        print_tree(x.right_node);
    }

    None => return
    }
}
fn main() {
    println!("Hello, world!");
    let left_leave: Option<Box<Node<i32>>> = Some(Box::new(Node{ value: 1, left_node: None, right_node: None }));
    let right_leave: Option<Box<Node<i32>>> = Some(Box::new(Node{ value: 2, left_node: None, right_node: None }));
    let tree: Option<Box<Node<i32>>> = Some(Box::new(Node{ value: 3,  left_node: left_leave, right_node: right_leave }));
    print_tree(tree);

}
