// Напишите функцию, которая принимает строку в качестве входных данных и возвращает перевернутую строку.

pub fn reverse(s: String) -> String {
    s.chars().rev().collect()
}

#[test]
fn hello() {
    assert_eq!(reverse("Hello".to_string()), String::from("olleH"));
}

#[test]
fn live() {
    let s = String::from("live on time , emit no evil");
    assert_eq!(reverse(s.clone()), s);
}
