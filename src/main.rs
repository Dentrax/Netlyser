extern crate netlyser;

use netlyser::{cli, error, run};

fn main() -> error::Result<()> {
    run(cli::get_args()?)
}
