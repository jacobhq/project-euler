fn main() {
    let mut count = 1;
    let mut num = 3;

    while count < 10001 {
        if is_prime(num) {
            count += 1;
        }
        num += 2;
    }

    println!("{}", num - 2);
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
