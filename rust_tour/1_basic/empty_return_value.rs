fn make_nothing() -> () {
    return ();
}

// 戻り値が()と推論
fn make_nothing2() {

}

fn main() {
    let a = make_nothing();
    let b = make_nothing();

    // 空を表示するのは難しいので、
    // a と b のデバッグ文字列を表示
    println!("a の値: {:?}", a);
    println!("b の値: {:?}", b);
}