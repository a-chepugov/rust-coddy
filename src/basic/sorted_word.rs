// принимает word (строку) и возвращает это слово с буквами, отсортированными по алфавиту.
fn sortString(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

#[test]
fn test() {
  assert_eq!(sortString("test".to_string()), "estt");
  assert_eq!(sortString("hello".to_string()), "ehllo");

}