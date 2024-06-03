use std::fs;
use std::path::PathBuf;

pub fn ex01() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2015/01.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2015-01a: {}", ex01a(&inp));
    println!("2015-01b: {}", ex01b(&inp).unwrap())
}

fn ex01a(inp: &str) -> i64 {
    let mut floor = 0;

    for c in inp.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    floor
}

fn ex01b(inp: &str) -> Option<i64> {
    let mut floor = 0;

    for (i, c) in inp.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return Some((i as i64) + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex01a() {
        let mut inp;

        inp = "(())";
        assert_eq!(ex01a(inp), 0);

        inp = "()()";
        assert_eq!(ex01a(inp), 0);

        inp = "(((";
        assert_eq!(ex01a(inp), 3);

        inp = "(()(()(";
        assert_eq!(ex01a(inp), 3);

        inp = "))(((((";
        assert_eq!(ex01a(inp), 3);

        inp = "())";
        assert_eq!(ex01a(inp), -1);

        inp = "))(";
        assert_eq!(ex01a(inp), -1);

        inp = ")))";
        assert_eq!(ex01a(inp), -3);

        inp = ")())())";
        assert_eq!(ex01a(inp), -3);
    }

    #[test]
    fn test_ex01b() {
        let mut inp;

        inp = ")";
        assert_eq!(ex01b(inp), Some(1));

        inp = "()())";
        assert_eq!(ex01b(inp), Some(5));

        inp = "(";
        assert_eq!(ex01b(inp), None);
    }
}
