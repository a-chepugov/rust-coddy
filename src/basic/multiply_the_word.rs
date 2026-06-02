// Напишите функцию с именем mulWord, которая принимает строку s и целое число n и возвращает строку n раз с пробелом между ними.

fn mulWord(s: String, n: i32) -> String {
    if n > 0 {
        let mut r = String::with_capacity(s.len() * n as usize + n as usize - 1);
        for _ in 0 .. (n - 1) {
            r.push_str(&s);
            r.push_str(" ");
        }
        r.push_str(&s);
        r
    } else {
        String::from("")
    }
}

#[test]
fn test() {
    assert_eq!(&mulWord("cow".to_string(), 3), "cow cow cow");
}

