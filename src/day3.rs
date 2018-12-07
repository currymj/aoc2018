use regex::{Regex, Error};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct ElfClaim {
    id: i32,
    topleft: (i32, i32),
    size: (i32, i32)
}

impl ElfClaim {
    fn all_coords(&self) -> Vec<(i32, i32)> {
        let curr_coord = self.topleft;
        let size = self.size;
        let mut result_vec = Vec::new();
        for x in curr_coord.0..(curr_coord.0 + size.0) {
            for y in curr_coord.1..(curr_coord.1 + size.1) {
                result_vec.push((x, y));
            }
        }
        result_vec
    }
}
fn parse_claim(s: &str) -> Option<ElfClaim> {
    let re = Regex::new(r"#(?P<claimnum>\d+) @ (?P<topx>\d+),(?P<topy>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
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

fn parse_claims(s: &[&str]) -> Vec<ElfClaim> {
    s.iter().flat_map(| &s| {parse_claim(s)}).collect()
}

pub fn find_overlaps(s: &[&str]) -> usize {
    let parsed_structs = parse_claims(s);
    let mut seen_once_set = HashSet::new();
    let mut seen_twice_set = HashSet::new();

    for claim in parsed_structs {
        let included_coords = claim.all_coords();
        for coord in included_coords {
            if seen_once_set.contains(&coord){
                seen_twice_set.insert(coord);
            }
            seen_once_set.insert(coord);
        }
    }
    seen_twice_set.len()
}

#[cfg(test)]
mod tests {
    use super::{parse_claim, find_overlaps};
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

    #[test]
    fn aoc_testcase() {
        let tests = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#34 @ 55,55: 22x22"];
        let overlapping_area = find_overlaps(&tests);
        assert_eq!(overlapping_area, 4);
    }
}
