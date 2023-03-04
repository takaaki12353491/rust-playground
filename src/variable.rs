pub fn _mutable() {
    // mutをつけないと変更できない
    let mut x = 1;
    println!("x is {}", x);
    x = 2;
    println!("x is {}", x);
}
