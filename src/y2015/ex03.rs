use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn main() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2015/03.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2015-03a: {}", ex_a(&inp));
    println!("2015-03b: {}", ex_b(&inp));
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

fn ex_a(inp: &str) -> u64 {
    let mut pos = Coordinate { x: 0, y: 0 };
    let mut map = HashMap::new();
    map.insert(pos, 1);
    for c in inp.chars() {
        match c {
            '^' => pos.y += 1,
            '>' => pos.x += 1,
            'v' => pos.y -= 1,
            '<' => pos.x -= 1,
            _ => {}
        }
        let count = map.entry(pos).or_insert(0);
        *count += 1;
    }
    map.values().count() as u64
}

fn ex_b(inp: &str) -> u64 {
    let mut pos = [Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }];
    let mut map = HashMap::new();
    map.insert(pos[0], 1);
    for (i, c) in inp.chars().enumerate() {
        let i = i % 2;
        match c {
            '^' => pos[i].y += 1,
            '>' => pos[i].x += 1,
            'v' => pos[i].y -= 1,
            '<' => pos[i].x -= 1,
            _ => {}
        }
        let count = map.entry(pos[i]).or_insert(0);
        *count += 1;
    }
    map.values().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_a() {
        let mut inp;

        inp = ">";
        assert_eq!(ex_a(inp), 2);

        inp = "^>v<";
        assert_eq!(ex_a(inp), 4);

        inp = "^v^v^v^v^v";
        assert_eq!(ex_a(inp), 2);
    }

    #[test]
    fn test_ex_b() {
        let mut inp;

        inp = "^v";
        assert_eq!(ex_b(inp), 3);

        inp = "^>v<";
        assert_eq!(ex_b(inp), 3);

        inp = "^v^v^v^v^v";
        assert_eq!(ex_b(inp), 11);
    }
}
