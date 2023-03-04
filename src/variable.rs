// constをつけると定数になる
// 定数は型が必要
// 全て大文字で書く
const _CON: i32 = 1;

pub fn _mutable() {
    // mutをつけないと変更できない
    let mut x = 1;
    println!("x is {}", x);
    x = 2;
    println!("x is {}", x);
}

pub fn _pointer() {
    let mut x = 1;
    // &をつけるとアドレスを取得できる
    // {:p}はポインタ
    println!("Memory address is {:p}", &x);
    // 再代入するとアドレスが変わる
    x = x + 1;
    println!("Memory address is {:p}", &x);
}
