use structopt::StructOpt;
use std::char;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "SHIFT")]
    shift: String,
    #[structopt(name = "MESSAGE")]
    message: String,
}

fn main() {
    let args = Opt::from_args();
    let shift: u32 = args.shift.parse().unwrap(); 
    println!("{}", shift);
    println!("{}", args.message);
    let cv: Vec<char> = args.message.chars().collect();
    for c in cv {
        println!("{}", rotate(c, shift));
    }
    return ();
}

fn rotate(c: char, n: u32) -> char {
    let f = match c.to_digit(36) {
        Some(v) => (v - 9 + n) % 26 + 9,
        None => 0
    };
    let r = match char::from_digit(f, 36) {
        Some(x) => x,
        None => '?',
    };
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_0() {
        let result = rotate ('u', 0);
        assert_eq!(result, 'u');
    }
    #[test]
    fn rotate_1() {
        let result = rotate ('u', 1);
        assert_eq!(result, 'v');
    }
    #[test]
    fn rotate_last() {
        let result = rotate ('z', 1);
        assert_eq!(result, 'a');
    }
}
