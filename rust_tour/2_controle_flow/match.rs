fn main() {
    let x = 42;

    // match は網羅的なので、すべてのケースを処理しなければなりません。
    match x {
        0 => {
            println!("found zero!");
        }
        1 | 2 => {
            println!("found 1 or 2!");
        }
        3..=9 => {
            println!("found a number 3 to 9");
        }
        // マッチした数字を変数に束縛
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // 必須
        // どのパターンにもマッチしない場合
        _ => {
            println!("found something else!");
        }
    }
}