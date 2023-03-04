pub fn _dangling_pointer() {
    let _r;
    {
        let x = 5;
        _r = &x;
    }
    // 参照rは実態xより長生きしてはいけない
    //println!("r is {}", r);
}

// 'aは短い方のライフタイムを受け取るアノテーション
fn _get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        y
    } else {
        x
    }
}
