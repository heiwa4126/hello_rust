pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
fn sub(x: i32, y: i32) -> i32 {
    x - y
}

#[cfg(test)]
mod tests {
    // ここはhello_rust::calc::tests
    use super::*;
    // ↑ hello_rust::calc::の関数を全部とりこむ
    #[test]
    fn it_works() {
        println!("calc::tests::it_works()");
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(2, 2), 4);
        assert_eq!(sub(2, 1), 1);
    }
}
