// Напишите функцию с именем removeAt, которая принимает строку и целое число и возвращает строку без символа по индексу, соответствующему этому числу.
// `Example`, 3 -> `Exaple`

pub fn removeAt(s: String, i: i32) -> String {
    if s.len() > i as usize {
        let mut s = s;
        s.remove(i as usize);
        s
    } else {
        s
    }
}

#[test]
fn test() {
    assert_eq!(&removeAt("bob".to_string(), 1), "bb");
}
