// if、match、関数、ブロックの最後が ; のない式であれば、戻り値として使用されます。
fn example() -> i32 {
    let x = 42;

    // 三項演算子
    let v = if x < 4 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグです",
        _ => "ホットドッグではありません"
    };
    println!("食品の識別: {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);

    v + 4
}

fn main() {
    println!("関数より: {}", example());
}