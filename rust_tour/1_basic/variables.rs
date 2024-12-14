fn main() {
    // 変数は基本的に小文字のスネークケース
    // snake_case

    
    // 型の推論
    let x = 13;
    println!("{}", x);
    
    // 型を指定
    let x: f64 = 3.141592654;
    println!("{}", x);

    // 宣言後で初期化
    let x;
    x = 0;
    println!("{}", x);
}