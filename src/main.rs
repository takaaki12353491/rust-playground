// modで読み込まないとrust-analyzerが効かない場合がある
mod module;
mod variable;

fn main() {
    // !はマクロ
    println!("Hello, world!");
}

// 定義のみで使用しない関数や変数には先頭に_をつける
fn _unused() {
    let _x = 1;
}
