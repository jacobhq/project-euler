fn main() {
    let mut largest_palindrome = 0;

    for i in 0..999 {
        for j in 0..999 {
            let product = i * j;
            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    println!("{}", largest_palindrome);
}

fn is_palindrome(mut n: i32) -> bool {
    let original = n;
    let mut reversed_integer = 0;
    let mut rem = 0;

    while n > 0 {
        rem = n % 10;
        n /= 10;
        reversed_integer = reversed_integer * 10 + rem;
    }

    reversed_integer == original
}
