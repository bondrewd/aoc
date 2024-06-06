use std::fs;
use std::path::PathBuf;

pub fn ex06() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2015/06.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2015-06a: {}", ex06a(&inp));
    println!("2015-06b: {}", ex06b(&inp));
}

struct Rectangle {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Rectangle {
    fn from_coordinates(coord1: &str, coord2: &str) -> Self {
        let tokens: Vec<_> = coord1.split(',').filter_map(|s| s.parse().ok()).collect();

        let x1 = tokens[0];
        let y1 = tokens[1];

        let tokens: Vec<_> = coord2.split(',').filter_map(|s| s.parse().ok()).collect();

        let x2 = tokens[0];
        let y2 = tokens[1];

        Rectangle { x1, y1, x2, y2 }
    }
}

enum Command {
    On(Rectangle),
    Off(Rectangle),
    Toggle(Rectangle),
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.split_whitespace().collect::<Vec<&str>>()[..] {
            ["turn", "on", coord1, "through", coord2] => {
                Ok(Command::On(Rectangle::from_coordinates(coord1, coord2)))
            }
            ["turn", "off", coord1, "through", coord2] => {
                Ok(Command::Off(Rectangle::from_coordinates(coord1, coord2)))
            }
            ["toggle", coord1, "through", coord2] => {
                Ok(Command::Toggle(Rectangle::from_coordinates(coord1, coord2)))
            }
            _ => Err(format!("Invalid string '{s}'")),
        }
    }
}

fn ex06a(inp: &str) -> u64 {
    let rows = 1000;
    let cols = 1000;
    let mut grid = vec![vec![0_u64; cols]; rows];

    for command in inp.lines().filter_map(|line| Command::try_from(line).ok()) {
        match command {
            Command::On(r) => {
                for row in grid.iter_mut().take(r.x2 + 1).skip(r.x1) {
                    for g in row.iter_mut().take(r.y2 + 1).skip(r.y1) {
                        *g = g.saturating_add(1);
                    }
                }
            }
            Command::Off(r) => {
                for row in grid.iter_mut().take(r.x2 + 1).skip(r.x1) {
                    for g in row.iter_mut().take(r.y2 + 1).skip(r.y1) {
                        *g = g.saturating_sub(1);
                    }
                }
            }
            Command::Toggle(r) => {
                for row in grid.iter_mut().take(r.x2 + 1).skip(r.x1) {
                    for g in row.iter_mut().take(r.y2 + 1).skip(r.y1) {
                        *g = 1 - *g;
                    }
                }
            }
        }
    }

    grid.iter().map(|v| v.iter().sum::<u64>()).sum()
}

fn ex06b(inp: &str) -> u64 {
    let rows = 1000;
    let cols = 1000;
    let mut grid = vec![vec![0_u64; cols]; rows];

    for command in inp.lines().filter_map(|line| Command::try_from(line).ok()) {
        match command {
            Command::On(r) => {
                for row in grid.iter_mut().take(r.x2 + 1).skip(r.x1) {
                    for g in row.iter_mut().take(r.y2 + 1).skip(r.y1) {
                        *g = g.saturating_add(1);
                    }
                }
            }
            Command::Off(r) => {
                for row in grid.iter_mut().take(r.x2 + 1).skip(r.x1) {
                    for g in row.iter_mut().take(r.y2 + 1).skip(r.y1) {
                        *g = g.saturating_sub(1);
                    }
                }
            }
            Command::Toggle(r) => {
                for row in grid.iter_mut().take(r.x2 + 1).skip(r.x1) {
                    for g in row.iter_mut().take(r.y2 + 1).skip(r.y1) {
                        *g = g.saturating_add(2);
                    }
                }
            }
        }
    }

    grid.iter().map(|v| v.iter().sum::<u64>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex06a() {
        let mut inp;

        inp = "turn on 0,0 through 999,999";
        assert_eq!(ex06a(inp), 1000 * 1000);

        inp = "toggle 0,0 through 999,0";
        assert_eq!(ex06a(inp), 1000);

        inp = "turn off 499,499 through 500,500";
        assert_eq!(ex06a(inp), 0);
    }

    #[test]
    fn test_ex06b() {
        let mut inp;

        inp = "turn on 0,0 through 0,0";
        assert_eq!(ex06b(inp), 1);

        inp = "toggle 0,0 through 999,999";
        assert_eq!(ex06b(inp), 2000000);
    }
}
