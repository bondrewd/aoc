use md5;
use std::fs;
use std::path::PathBuf;

pub fn main() {
    let mut inp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    inp_path.push("inp/2015/04.inp");

    let inp = fs::read_to_string(inp_path).unwrap();
    println!("2015-04a: {}", ex_a(&inp).unwrap());
    println!("2015-04b: {}", ex_b(&inp).unwrap());
}

fn find_number(inp: &str, header: &str) -> Option<u64> {
    let inp = inp.split_whitespace().collect::<String>();
    for i in 0.. {
        let hash = inp.clone() + i.to_string().as_str();
        let hash = md5::compute(hash);
        let hash = format!("{hash:x}");
        if hash.starts_with(header) {
            return Some(i);
        }
    }
    None
}
fn ex_a(inp: &str) -> Option<u64> {
    find_number(inp, "00000")
}

fn ex_b(inp: &str) -> Option<u64> {
    find_number(inp, "000000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_a() {
        let mut inp;

        inp = "abcdef";
        assert_eq!(ex_a(inp), Some(609043));

        inp = "pqrstuv";
        assert_eq!(ex_a(inp), Some(1048970));
    }
}
