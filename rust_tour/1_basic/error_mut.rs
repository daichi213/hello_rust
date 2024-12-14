fn main() {
    // 不変 - コンパイラは変数の読み込みだけを許可します。
    let x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}