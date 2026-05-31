fn fibo(i: i32) -> i32 {
    let mut shift = 1;
    let mut curr = 0;
    for _ in 0..i {
        let temp = curr;
        curr += shift;
        shift = temp;
    }
    curr
}

#[test]
fn test0() {
    assert_eq!(fibo(0), 0);
}


#[test]
fn test3() {
    assert_eq!(fibo(3), 2);
}

#[test]
fn test7() {
    assert_eq!(fibo(7), 13);
}

#[test]
fn test23() {
    assert_eq!(fibo(23), 28657);
}

