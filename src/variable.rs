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

pub fn _scope() {
    let x = 1;
    println!("Memory address is {}", x);
    // スコープで変数は独立
    {
        let x = 2;
        println!("Memory address is {}", x);
    }
}

pub fn _tuple() {
    // ()でタプルを作れる
    let t = (1, 0.5, "x");

    // ()でタプルから値を取得できる
    let (x, y, z) = t;
    println!("t values is {} {} {}", x, y, z);

    // .0などつけることで値を取得できる
    println!("t values is {} {} {}", t.0, t.1, t.2);

    // ネストできる
    let mut t2 = ((0, 1), (2, 3));
    // refでポインタを取得できる
    let ((ref mut px1, ref _px2), _) = t2;
    // *でポインタの値を参照できる
    *px1 = 4;
    // {:?}で複雑なデータ型を表示できる
    println!("t2 values is {:?}", t2);
}
