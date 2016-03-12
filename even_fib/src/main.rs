fn main() {
    println!("Starting Even Fibonacci");

    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;

    loop {
        if b % 2 == 0 {
            print!("{}, ", b);
            sum += b;
        }
        next_fib(&mut a, &mut b);

        if b > 4000000 {
            break;
        }
    }

    println!("\nSum: {}", sum);

}

fn next_fib(a: &mut u64, b: &mut u64) {
    let sum = *a + *b;
    *a = *b;
    *b = sum;
}
