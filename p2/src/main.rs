fn main() {
    let mut a = 2u64;
    let mut b = 8u64;
    let mut s = 2u64;

    while b < 4_000_000 {
        s += b;
        let next = 4 * b + a;
        a = b;
        b = next;
    }
    
    println!("{}", s)
}
