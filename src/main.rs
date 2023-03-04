// modで読み込まないとrust-analyzerが効かない場合がある
mod r#enum;
mod generics;
mod lifetime;
mod memory;
mod module;
mod owenership;
mod pointer;
mod r#struct;
mod r#trait;
mod r#type;
mod variable;

fn main() {
    // !はマクロ
    println!("Hello, world!");
}

// 定義のみで使用しない関数や変数には先頭に_をつける
fn _unused() {
    let _x = 1;
}
