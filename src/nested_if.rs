fn main() {
    'outer:for i in 1..11 {
        println!("Multiplication table for {} is:-", i);
        'inner:for j in 1..11 {
            if i == 3 {continue 'outer;};
            println!("{} * {} = {}", i, j, i * j);
        }
    }
}