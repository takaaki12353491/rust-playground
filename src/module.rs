// pubをつけるとサブモジュールを公開できる
pub mod sub_module;

// 公開できない
fn _private() {
    println!("private");
}

// pubをつけると公開できる
pub fn _public() {
    println!("public");
}
