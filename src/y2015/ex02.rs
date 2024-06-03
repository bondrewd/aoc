use std::fs;
use std::path::PathBuf;

pub fn ex02() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2015/02.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2015-02a: {}", ex02a(&inp));
    println!("2015-02b: {}", ex02b(&inp))
}

struct Dimensions {
    w: u64,
    h: u64,
    l: u64,
}

impl Dimensions {
    fn paper_area(&self) -> u64 {
        let wh = self.w * self.h;
        let wl = self.w * self.l;
        let hl = self.h * self.l;
        let ex = u64::MAX.min(wh).min(wl).min(hl);

        2 * (wh + wl + hl) + ex
    }

    fn ribbon_length(&self) -> u64 {
        let wh = self.w + self.h;
        let wl = self.w + self.l;
        let hl = self.h + self.l;
        let ex = u64::MAX.min(wh).min(wl).min(hl);

        2 * ex + self.w * self.h * self.l
    }
}

impl From<&str> for Dimensions {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 3 {
            panic!("Invalid input format");
        }

        let w = parts[0].parse::<u64>().expect("Invalid width");
        let h = parts[1].parse::<u64>().expect("Invalid height");
        let l = parts[2].parse::<u64>().expect("Invalid depth");

        Dimensions { w, h, l }
    }
}

fn ex02a(inp: &str) -> u64 {
    inp.lines()
        .map(Dimensions::from)
        .map(|d| d.paper_area())
        .reduce(|a, b| a + b)
        .unwrap()
}

fn ex02b(inp: &str) -> u64 {
    inp.lines()
        .map(Dimensions::from)
        .map(|d| d.ribbon_length())
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex02a() {
        let mut inp;

        inp = "2x3x4";
        assert_eq!(ex02a(inp), 58);

        inp = "1x1x10";
        assert_eq!(ex02a(inp), 43);
    }

    #[test]
    fn test_ex02b() {
        let mut inp;

        inp = "2x3x4";
        assert_eq!(ex02b(inp), 34);

        inp = "1x1x10";
        assert_eq!(ex02b(inp), 14);
    }
}
