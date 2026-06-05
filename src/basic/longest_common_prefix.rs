fn longestCommonPrefix(a: Vec<String>) -> String {
    let mut prefix = &a[0][..];
    for str in &a[1..] {
        let mut pchars = prefix.chars();
        let mut schars = str.chars();
        let mut len = 0;
        while let (Some(pchar), Some(schar)) = (pchars.next(), schars.next()) {
            if pchar == schar {
                len += 1;
            } else {
                prefix = &prefix[0..len];
                break;
            }
        }
    }

    prefix.to_string()
}

#[test]
fn test() {
    assert_eq!(longestCommonPrefix(vec!["Hello, Bob".to_string(), "Hello, Jake".to_string()]), "Hello, ");
}