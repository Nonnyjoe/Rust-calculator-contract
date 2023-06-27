use std::u128;

fn main() {
    let numa: u128 = 11;
    let numb: u128 = 37;
    println!("{}", add(numa, numb));
    println!("{}", subtract(numa, numb));
    println!("{}", divide(numa, numb));
    println!("{}", multiply(numa, numb));
}
fn add(num1: u128, num2: u128) -> u128 {
    return num1 + num2;
}

fn subtract(num1: u128, num2: u128) -> u128 {
    return num1 - num2;
}

fn divide(num1: u128, num2: u128) -> u128 {
    return num1 / num2;
}
fn multiply(num1: u128, num2: u128) -> u128 {
    return num1 * num2;
}
