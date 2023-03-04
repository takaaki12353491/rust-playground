pub fn _copy() {
    let i1 = 1;
    // static memoryを参照する型はコピーできるのでi1が使える
    let i2 = i1;
    println!("i1 value is {}", i1);
    println!("i2 value is {}", i2)
}

pub fn _not_copy() {
    let s1 = String::from("hello");
    // heap memoryを所有する型はコピーできないのでs1が使えない
    let s2 = s1;
    println!("s2 value is {}", s2);
}

pub fn _deep_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // s1とs2は異なるアドレスになる
    println!("s1 stack address value is {:p}", &s1);
    println!("s1 stack address value is {:p}", &s2);
}

// 呼び出し元でまた使える
fn _giveback(s: String) -> String {
    s
}

// アドレスを渡すことで処理だけ分割できる
fn _operate(s: &String) -> usize {
    s.len()
}

// mutableだと値を操作できる
fn _change(s: &mut String) {
    s.push_str(" world");
}
