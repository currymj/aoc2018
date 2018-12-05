use regex::{Regex, Error};


#[derive(Debug, PartialEq, Eq)]
struct ElfClaim {
    id: i32,
    topleft: (i32, i32),
    size: (i32, i32)
}

fn parse_claim(s: &str) -> Option<ElfClaim> {
    let re = Regex::new(r"#(?P<claimnum>\d+) @ (?P<topx>\d),(?P<topy>\d): (?P<width>\d)x(?P<height>\d)").unwrap();
    let caps = re.captures(s)?;

    // bad error handling!
    Some(
        ElfClaim {
            id: caps["claimnum"].parse().unwrap(),
            topleft: (caps["topx"].parse().unwrap(), caps["topy"].parse().unwrap()),
            size: (caps["width"].parse().unwrap(), caps["height"].parse().unwrap())
        }
    )
}

#[cfg(test)]
mod tests {
    use super::parse_claim;
    use super::ElfClaim;
    #[test]
    fn test1() {
        let test_str = "#123 @ 3,2: 5x4";
        let result = parse_claim(test_str).unwrap();
        assert_eq!(result, ElfClaim {
            id: 123,
            topleft: (3, 2),
            size: (5, 4)
        });

    }
}
