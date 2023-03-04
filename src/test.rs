#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn is_larger(&self, other: &Rectangle) -> bool {
        other.width * other.height < self.width * self.height
    }
}

#[allow(dead_code)]
fn double(x: i32) -> i32 {
    x * 2
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a_is_larger() {
        let a = Rectangle {
            width: 5,
            height: 5,
        };
        let b = Rectangle {
            width: 3,
            height: 3,
        };
        assert!(a.is_larger(&b));
    }

    #[test]
    fn test_double() {
        assert_eq!(6, double(3))
    }

    #[test]
    fn test_contains_name() {
        let res = greeting("Rust");
        assert!(res.contains("Rust"));
    }
}
