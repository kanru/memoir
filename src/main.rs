extern crate docopt;
extern crate rustc_serialize;

pub mod config;

use docopt::Docopt;

#[cfg_attr(rustfmt,rustfmt_skip)]
const USAGE: &'static str = "
Usage: memoir build
       memoir -h

Options:
    -h, --help    Print this help message.
";

#[derive(RustcDecodable)]
struct Args {
    cmd_build: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|dopt| dopt.decode())
                         .unwrap_or_else(|e| e.exit());

    if args.cmd_build {
        let config = config::Config::new();
        println!("{:?}", config);
    }
}
