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
