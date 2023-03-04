pub fn _stack_overflow() {
    // 8MBをこえるとスタックオーバーフローになる
    let _a: [u8; 9000000] = [1; 9000000];
}
