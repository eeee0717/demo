fn test() -> i32 {
    let mut a = 1;
    let b = 12;
    a += 1;
    let c = loop {
        a += 1;
        if a == b {
            break a;
        }
    };

    a + c
}
fn main() {
    let a = test();
    println!("Hello, world!{:?}", a);
    println!("Hello, world!{:?}", a);
}
mod test {
    #[test]
    fn test_test() {
        let a = crate::test();
        assert_eq!(a, 24);
    }
    #[test]
    fn test_failed_test() {
        let a = crate::test();
        assert_ne!(a, 3);
    }
}
