use std::io;
use rand
fn main() {

    let first = take_str_ret_int("Input the first number:-");
    let second = take_str_ret_int("Input the second number");
    let third = take_str_ret_int("Input the third number");
    let fourth = take_str_ret_int("Input the fourth number");
    let fifth = take_str_ret_int("Input the fifth number");


    let mut arr:[i32;5] = [first, second, third, fourth, fifth];

    for i in 1..arr.len() {
        arr[i] = arr[i] * arr[i];
    }
    println!("{:?}", arr);
}

fn take_str_ret_int(s: &str) -> i32 {
    println!("{}", s);
    let mut num = String::new();
    io::stdin().read_line(&mut num)
                .expect("Failed to take user input :(");
    let num = num.trim().parse().expect("Please input a valid number");
    return num
}