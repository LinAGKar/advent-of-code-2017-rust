// Hardcoded solution for my input


fn main() {
    let mut number_of_non_primes = 0;
    let mut curr = 109300;
    let stop = curr + 17000;
    while curr <= stop {
        if (2..(curr as f64).sqrt() as i64 + 1).any(|x| curr % x == 0) {
            number_of_non_primes += 1;
        }
        curr += 17;
    }
    println!("{}", number_of_non_primes);
}
