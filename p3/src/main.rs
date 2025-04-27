fn main() {
    let n: u64 = 600851475143;
    let mut p = Vec::new();

    for i in 2..n {
        if is_fact(i, n) && is_prime(i) {
            println!("{i}");
            p.push(i);
        }
    }

    p.sort();
    println!("{:?}", p.last().unwrap());
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_fact(a: u64, b: u64) -> bool {
    b % a == 0
}
