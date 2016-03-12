fn main() {
    let max = 100000;
    let mut sum: u64 = 0;

    for n in 1..max {
        match n {
            n if n % 15 == 0    => sum += n,
            n if n % 5 == 0     => sum += n,
            n if n % 3 == 0     => sum += n,
            _                   => (),
        }
    }

    // done
    println!("Sum of multiples to {}: {}", max, sum);
}
