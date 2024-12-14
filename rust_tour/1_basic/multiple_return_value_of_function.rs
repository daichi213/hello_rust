fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    let result = swap(21, 22);
    println!("{} {}", result.0, result.1);

    let (a, b) = swap(1,2);
    println!("{} {}", a, b);
}