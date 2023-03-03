// pubをつけるとサブモジュールを公開できる
pub mod sub_module;

// 公開できない
fn private() {
    println!("private");
}

// pubをつけると公開できる
pub fn public() {
    println!("public");
}
