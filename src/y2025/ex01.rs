use std::fs;
use std::path::PathBuf;

pub fn main() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2025/01.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2025-01a: {}", ex_a(&inp));
    println!("2025-01b: {}", ex_b(&inp));
}

fn ex_a(inp: &str) -> i64 {
    let mut dial = 50;
    let mut zeroes = 0;

    for line in inp.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let (direction, number) = line.split_at_checked(1).unwrap();
        let number = number.parse::<i64>().unwrap();
        let direction = match direction {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };

        dial += direction * number;

        while dial < 0 {
            dial += 100;
        }

        while dial > 99 {
            dial -= 100;
        }

        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn ex_b(inp: &str) -> i64 {
    let mut dial: i64 = 50;
    let mut zeroes = 0;

    for line in inp.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (direction, number) = line.split_at_checked(1).unwrap();
        let number = number.parse::<i64>().unwrap();

        match direction {
            "R" => {
                zeroes += (dial + number) / 100;
                dial = (dial + number) % 100;
            }
            "L" => {
                zeroes += number / 100;

                let remainder = number % 100;

                if dial > 0 && remainder >= dial {
                    zeroes += 1;
                }

                dial = (dial - remainder + 100) % 100;
            }
            _ => unreachable!(),
        };
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_a() {
        let inp = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(ex_a(inp), 3);
    }

    #[test]
    fn test_ex_b() {
        let inp = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(ex_b(inp), 6);
    }
}
