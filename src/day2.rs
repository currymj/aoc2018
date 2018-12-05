use std::collections::{HashMap, HashSet};


fn remove_letter(s: &str, n: usize) -> String {
    let mut result = String::new();
    result.push_str(&s[0..n]);
    result.push_str(&s[n+1..]);
    result
}

fn find_duplicates(list: &[String]) -> HashMap<String, i32> {
    let mut result = HashMap::new();

    for s in list.into_iter() {
        let entry = result.entry(s.clone()).or_insert(0);
        *entry += 1;
    }
    result
}
fn find_matching_letters(list: &[&str]) -> Option<String> {
    let first_removed_list: Vec<String> = list.iter().map(|&s| {remove_letter(s, 0)}).collect();
    let dup_map = find_duplicates(&first_removed_list);
    for (key, value) in dup_map {
        if value > 1 {
            return Some(key);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::remove_letter;
    use super::find_matching_letters;
    #[test]
    fn select_char_test() {
        let input = "abcde";
        assert_eq!(remove_letter(input, 0), "bcde");
        assert_eq!(remove_letter(input, 1), "acde");
    }

    #[test]
    fn find_duplicate_when_first() {
        let input = ["bbcde", "abcde"];
        let result = find_matching_letters(&input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "bcde");
    }
}