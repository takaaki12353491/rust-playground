// 'aは短い方のライフタイムを受け取るアノテーション
fn _get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        y
    } else {
        x
    }
}
