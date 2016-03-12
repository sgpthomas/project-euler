/// This is how the program works generally
///
/// Plug number into `find_smallest_prime_factor(number)`
///    - This function starts at 2 and increases until it finds the first value
///      where the `number / divisor` remainder is 0 and then returns this divisor
/// Take the return value of `find_smallest_prime_factor` and puts it in an array
/// Then plug the original number/factor back into `find_smallest_prime_factor()`
/// and then repeat until the number gets to one
///
/// This is how the program works specifically for `number = 13195`
/// 13195 / 5 -> 2639
/// 2639 / 7 -> 377
/// 377 / 13 -> 29
/// 29 / 29 -> 1
///
/// So the prime factors for 13195 are (5, 7, 13, 29) but they were found just be looking
/// for the smallest generic divisor of (13195, 2639, 377, 29)
///
/// Note that this doesn't work for everything but it works for 600851475143

fn main() {
    println!("Largest Prime Factors");

    let mut number = 600851475143;
    let mut factors: Vec<u64> = vec![];

    loop {
        if number <= 1 {
            break;
        }

        let fac = find_smallest_prime_factor(&number);
        number /= fac;
        factors.push(fac);
    }

    println!("{:?}", factors);
}


fn find_smallest_prime_factor(num: &u64) -> u64 {
    let mut ret: u64 = *num;

    for i in 2..*num {
        if num % i == 0 {
            ret = i;
            break;
        }
    }

    ret
}
