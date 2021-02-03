fn main() {
    let mut n :u32 = 3;

    for i in 1..n {
        n *= i;
    }
    println!("{}", n);
}