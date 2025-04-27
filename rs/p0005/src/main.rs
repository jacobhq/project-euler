fn main() {
    let mut done = false;
    let mut i = 1;

    while !done {
        let mut f = false;
        for j in 1..=20 {
            if i % j != 0 {
                f = true
            }
        }

        if !f {
            println!("{i}");
            done = true;
            break
        }

        i += 1;
    }
}
