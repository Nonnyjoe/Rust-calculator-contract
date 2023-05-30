use std::u128;

fn main() {
    let my_vectors: Vec<i32> = vec![1, 3, 4];
    let a: u16 = 25;
    let _b: u16 = a;
    let mut title = String::from("solana ");
    title.push_str("Bootcmap");
    let tlength: usize = title.len();
    let tcapacity: usize = title.capacity();
    let mut my_vector: Vec<i32> = Vec::new();
    my_vector = vec![12, 15, 18];
    let my_array:<i32> = [1, 2, 3];
    println!("{}", title);
    println!("title length = {}", tlength);
    println!("title capacity = {}", tcapacity);
    println!("{}", a);
    println!("Hello, world!");
    println!("my vector at position 2 is {}", my_vectors[2]);

    let numa: u128 = 11;
    let numb: u128 = 37;
    println!("{}", add(numa, numb));
    println!("{}", subtract(numa, numb));
    println!("{}", divide(numa, numb));
    println!("{}", multiply(numa, numb));
}
fn add(num1: u128, num2: u128) -> u128 {
    return (num1 + num2);
}

fn subtract(num1: u128, num2: u128) -> u128 {
    return (num1 - num2);
}

fn divide(num1: u128, num2: u128) -> u128 {
    return (num1 / num2);
}
fn multiply(num1: u128, num2: u128) -> u128 {
    return (num1 * num2);
}
