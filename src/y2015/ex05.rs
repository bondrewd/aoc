use std::fs;
use std::path::PathBuf;

pub fn ex05() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2015/05.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2015-05a: {}", ex05a(&inp));
    println!("2015-05b: {}", ex05b(&inp));
}

fn qa1(inp: &str) -> bool {
    inp.chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        })
        .sum::<u64>()
        >= 3
}

fn qa2(inp: &str) -> bool {
    inp.chars()
        .zip(inp.chars().skip(1))
        .any(|(c1, c2)| c1 == c2)
}

fn qa3(inp: &str) -> bool {
    inp.chars()
        .zip(inp.chars().skip(1))
        .all(|pair| !matches!(pair, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
}

fn ex05a(inp: &str) -> u64 {
    inp.lines()
        .filter_map(|line| {
            if qa1(line) && qa2(line) && qa3(line) {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

fn qb1(inp: &str) -> bool {
    inp.chars()
        .zip(inp.chars().skip(1))
        .enumerate()
        .any(|(i, (a, b))| {
            inp.chars()
                .zip(inp.chars().skip(1))
                .enumerate()
                .any(|(j, (c, d))| (j as i32) - (i as i32) >= 2 && a == c && b == d)
        })
}

fn qb2(inp: &str) -> bool {
    inp.chars().zip(inp.chars().skip(2)).any(|(a, b)| a == b)
}

fn ex05b(inp: &str) -> u64 {
    inp.lines()
        .filter_map(|line| {
            if qb1(line) && qb2(line) {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex05a() {
        let mut inp;

        inp = "aei";
        assert!(qa1(inp));

        inp = "xazegov";
        assert!(qa1(inp));

        inp = "aeiouaeiouaeiou";
        assert!(qa1(inp));

        inp = "xx";
        assert!(qa2(inp));

        inp = "abcdde";
        assert!(qa2(inp));

        inp = "aabbccdd";
        assert!(qa2(inp));

        inp = "ugknbfddgicrmopn";
        assert_eq!(ex05a(inp), 1);

        inp = "aaa";
        assert_eq!(ex05a(inp), 1);

        inp = "jchzalrnumimnmhp";
        assert_eq!(ex05a(inp), 0);

        inp = "haegwjzuvuyypxyu";
        assert_eq!(ex05a(inp), 0);

        inp = "dvszwmarrgswjxmb";
        assert_eq!(ex05a(inp), 0);
    }

    #[test]
    fn test_ex05b() {
        let mut inp;

        inp = "xyxy";
        assert!(qb1(inp));

        inp = "aabcdefgaa";
        assert!(qb1(inp));

        inp = "aaa";
        assert!(!qb1(inp));

        inp = "xyx";
        assert!(qb2(inp));

        inp = "efe";
        assert!(qb2(inp));

        inp = "aaa";
        assert!(qb2(inp));

        inp = "qjhvhtzxzqqjkmpb";
        assert_eq!(ex05b(inp), 1);

        inp = "xxyxx";
        assert_eq!(ex05b(inp), 1);

        inp = "uurcxstgmygtbstg";
        assert_eq!(ex05b(inp), 0);

        inp = "ieodomkazucvgmuy";
        assert_eq!(ex05b(inp), 0);
    }
}
