pub fn split_and_parse_string(str: &String) -> Vec<i32> {
    let split_vec: Vec<&str> = str.split("\n").collect();

    let nums= split_vec.into_iter()
        .flat_map(|s: &str| {s.parse()}).collect();

    nums
}

#[cfg(test)]
mod tests {
    use super::split_and_parse_string;
    #[test]
    fn parse_works_empty() {
        let st: String = String::from("test");
        let results = split_and_parse_string(&st);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn parse_works_some() {
        let st = String::from("1\n2\n3\n");
        let results = split_and_parse_string(&st);
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 2);
        assert_eq!(results[2], 3);
    }

}
