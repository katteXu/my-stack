use my_stack::Stack;

fn main() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{:?}", stack);
    let sum = stack.iter().sum::<i32>();
    println!("sum: {}", sum);
}
