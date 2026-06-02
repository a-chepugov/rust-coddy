pub fn isPrime(num: i32) -> bool {
    let pivot = (num as f64).sqrt() as i32;
    for i in 2..=pivot {
        if num % i == 0 {
            return false;
        }
    }
    true
}


#[test]
fn test() {
    assert!(isPrime(1));
    assert!(isPrime(2));
    assert!(isPrime(3));
    assert!(!isPrime(4));
    assert!(isPrime(5));
}
