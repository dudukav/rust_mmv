pub mod args;
pub mod errors;
pub mod fs_utils;
pub mod mmv;

use crate::args::clap::Parser;
use crate::args::CLI;
use crate::mmv::mmv;

fn main() {
    let args = CLI::parse();
    if let Err(error) = mmv(args) {
        println!("{:?}", error);
    }
}
