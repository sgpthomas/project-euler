

fn main() {
    let mut largest: u64 = 0;
    for a in 111..1000 {
        for b in 111..1000 {
            if check_palindrome(&(a * b)) {
                if (a * b) > largest { largest = a * b };
                // println!("{} * {} = {}", a, b, a * b);
            }
        }
    }

    println!("{}", largest);
}

fn check_palindrome(n: &u64) -> bool {
    let string = n.to_string();
    let mut new_string = String::new();

    for c in n.to_string().chars().rev() {
        new_string.push(c);
    }

    new_string == string
}
