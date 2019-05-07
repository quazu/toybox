use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "SHIFT")]
    shift: String,
    #[structopt(name = "MESSAGE")]
    message: String,
}

fn main() {
    let args = Opt::from_args();
    let shift: i32 = args.shift.parse().unwrap(); 
    println!("{}", shift);
    println!("{}", args.message);
}

fn rotate(_c: char, _n: i32) -> char {
    panic!("NotImplemented");
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
}
