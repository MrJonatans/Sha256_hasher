use clap::Parser;
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Number of zeros in end of sha256 code
    #[arg(short = 'F', long = "F", default_value_t = 0)]
    pub f: usize,

    /// Number of output values
    #[arg(short = 'N', long = "N", default_value_t = 1)]
    pub n: usize,

    /// Select function (false = zero counter, true = message hasher)
    #[arg(short = 's', long = "switch", default_value_t = false)]
    pub switch: bool,

    /// Input message to hash
    #[clap(
        short = 'm',
        long = "message",
        default_value = "emptiness, endless and boundless"
    )]
    pub message: String,
}

pub fn get_input() -> (usize, usize, bool, String) {
    let args: Args = Args::parse();
    return (args.f, args.n, args.switch, args.message);
}
