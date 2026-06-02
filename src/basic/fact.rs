/// Принимает num и возвращает факториал num
pub fn fact(num: i32) -> i32 {
    let mut result = 1;
    for i in 2..=num {
        result *= i;
    }
    result
}

#[test]
fn test() {
    assert_eq!(fact(1), 1);
    assert_eq!(fact(2), 2);
    assert_eq!(fact(3), 6);
    assert_eq!(fact(4), 24);
}
