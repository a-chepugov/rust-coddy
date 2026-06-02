// Напишите функцию с именем capitalize, которая принимает строку (только в нижнем регистре), делает заглавной первую букву каждого слова и выводит результат.
// Например, `this is an example` -> `This Is An Example`

pub fn capitalize(s: String) -> String {
    s
        .split_whitespace()
        .map(|word| {
            if word.len() > 0 {
                let mut chars = word.chars();
                let mut r = chars.next().unwrap().to_uppercase().to_string();
                r.push_str(chars.as_str());
                r
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn test() {
    assert_eq!(&capitalize("this is an example".to_string()), "This Is An Example");
}
