/// принимает целое число и возвращает true, если число является палиндромом, и false в противном случае
pub fn isPalindrome(num: i32) -> bool {
    let mut digits = vec![];
    let mut num = num;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    
    let len = digits.len();
    for i in 0 .. (len / 2) {
        if digits[i] != digits[len - i - 1] { return false; }
    }

    true
}

#[test]
fn test() {
    assert!(isPalindrome(1));
    assert!(isPalindrome(121));
    assert!(!isPalindrome(42));
}
