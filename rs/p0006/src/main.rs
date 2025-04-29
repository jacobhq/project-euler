fn main() {
    let hundred: Vec<i32> = (0..=100).collect();
    let hundred_sum: i32 = hundred.iter().sum();
    let squares: i32 = hundred.iter().map(|i| i * i).sum();
    let square_of_sum = hundred_sum * hundred_sum;
    let diff: i32 = square_of_sum - squares;
    println!("{diff}");
}
