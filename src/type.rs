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

pub fn _array() {
    // []で配列を作る
    let a = [1, 2, 3, 4, 5];
    // 0を10個の配列を作る
    let b = [0; 10];
    println!("a values is {:?}", a);
    // []で配列の要素を取り出せる
    println!("b0 values is {}", b[0]);
}

pub fn _string() {
    let s1 = "hello";
    // as_ptr()でstaticメモリのアドレスを取得できる
    println!("s1 static memory address is {:?}", s1.as_ptr());
    // len()で実データのbyteを取得できる
    println!("s length is {}", s1.len());

    // String型はheapに保存される
    let mut s2 = String::from("hello");
    println!("s2 stack address is {:?}", &s2);
    println!("s2 heap memory address is {:?}", s2.as_ptr());
    println!("s2 length is {:?}", s2.as_ptr());
    println!("s2 capacity is {:?}", s2.as_ptr());
    s2.push_str("_new");
    println!("new s2 value is {}", s2);
}

pub fn _vector() {
    // vec!でベクター型の値を作成できる。
    let mut v1 = vec![1, 2, 3, 4];
    let mut v2 = vec![5, 6, 7, 8];
    println!("v1 stack address is {:?}", &v1);
    println!("v1 heap memory address is {:?}", v1.as_ptr());
    println!("v1 length is {}", v1.len());
    println!("v1 capacity is {}", v1.capacity());
    v1.insert(1, 10);
    println!("v1 value is {:?}", v1);
    v1.remove(0);
    println!("v1 value is {:?}", v1);
    v1.append(&mut v2);
    println!("v1 value is {:?}", v1);
    // v3は空になる
    println!("v2 value is {:?}", v2);
}
