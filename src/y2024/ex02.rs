use std::fs;
use std::path::PathBuf;

pub fn ex02() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2024/02.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2024-01a: {}", ex02a(&inp));
    println!("2024-01b: {}", ex02b(&inp));
}

fn is_safe(report: Vec<i64>) -> bool {
    let is_increasing = report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(a, b)| 1 <= b - a && b - a <= 3);
    let is_decreasing = report
        .iter()
        .zip(report.iter().skip(1))
        .all(|(a, b)| 1 <= a - b && a - b <= 3);
    is_increasing || is_decreasing
}

fn is_tolerant_safe(report: Vec<i64>) -> bool {
    let report_is_safe = is_safe(report.clone());
    let report_is_tolerant_safe = (0..report.len()).any(|index| {
        is_safe(
            report
                .iter()
                .enumerate()
                .filter_map(|(i, &n)| if i != index { Some(n) } else { None })
                .collect(),
        )
    });
    report_is_safe || report_is_tolerant_safe
}

fn evaluate_safety(inp: &str, checker: fn(Vec<i64>) -> bool) -> u64 {
    inp.lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|number| number.parse().ok())
                .collect()
        })
        .map(checker)
        .filter(|&safe| safe)
        .count() as u64
}

fn ex02a(inp: &str) -> u64 {
    evaluate_safety(inp, is_safe)
}

fn ex02b(inp: &str) -> u64 {
    evaluate_safety(inp, is_tolerant_safe)
}

#[cfg(test)]
mod tests {
    use crate::y2024::ex02;
    #[test]
    fn test_2024ex02a() {
        let inp = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";
        assert_eq!(ex02::ex02a(inp), 2);
    }

    #[test]
    fn test_2024ex02b() {
        let inp = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";
        assert_eq!(ex02::ex02b(inp), 4);
    }
}
