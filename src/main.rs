fn main() {
    println!("{}", monotaci(10));
}

fn monotaci(value: i32) -> i32 {
    if value == 0 {
        return 0;
    } else if value == 1 {
        return 1;
    }
    let mut previous = 0;
    let mut current = 1;

    for _ in 1..value  {
        let next = previous + current;
        previous = current;
        current = next;
    }
    current
}
