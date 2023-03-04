use crate::memory::_stack_overflow;

// modで読み込まないとrust-analyzerが効かない場合がある
mod memory;
mod module;
mod owenership;
mod pointer;
mod r#type;
mod variable;

fn main() {
    // !はマクロ
    println!("Hello, world!");
    _stack_overflow();
}

// 定義のみで使用しない関数や変数には先頭に_をつける
fn _unused() {
    let _x = 1;
}
