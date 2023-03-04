#[allow(dead_code)]
enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn _os() {
    let windows = OS::Windows(1985, String::from("Microsoft"));
    print_os_info(windows);
    let mac = OS::Windows(1985, String::from("Apple"));
    print_os_info(mac);
    let linux = OS::Windows(1985, String::from("Linus"));
    print_os_info(linux);
}

#[allow(dead_code)]
fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, who) => {
            println!("Windows: First release in {} by {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("Windows: First release in {} by {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Windows: First release in {} by {}", year, who);
        }
    }
}
