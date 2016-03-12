fn main() {
    println!("Largest Prime Factors");

    let mut number = 600851475143;
    let mut factors: Vec<u64> = vec![];

    for _ in 0..10 {
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
