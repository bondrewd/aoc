use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn ex01() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2024/01.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2024-01a: {}", crate::y2024::ex01::ex01a(&inp));
    println!("2024-01b: {}", crate::y2024::ex01::ex01b(&inp));
}

fn ex01a(inp: &str) -> i64 {
    let mut col1 = vec![];
    let mut col2 = vec![];
    for line in inp.lines() {
        let numbers = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i64>>();
        col1.push(numbers[0]);
        col2.push(numbers[1]);
    }
    col1.sort_unstable();
    col2.sort_unstable();
    col1.iter().zip(col2).map(|(a, b)| (a - b).abs()).sum()
}

fn ex01b(inp: &str) -> i64 {
    let mut col1 = vec![];
    let mut col2 = HashMap::new();
    for line in inp.lines() {
        let numbers = line
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<i64>>();
        col1.push(numbers[0]);
        *col2.entry(numbers[1]).or_insert(0) += 1;
    }
    col1.iter().map(|n| n * col2.get(n).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2024ex01a() {
        let inp;

        inp = "3  4\n4  3\n2  5\n1  3\n3  9\n3  3\n";
        assert_eq!(ex01a(inp), 11);
    }

    #[test]
    fn test_2024ex01b() {
        let inp;

        inp = "3  4\n4  3\n2  5\n1  3\n3  9\n3  3\n";
        assert_eq!(ex01b(inp), 31);
    }
}
