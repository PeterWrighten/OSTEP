use clap::{App, Arg, SubCommand};
use std::io::Result<()>;

fn main() -> Result<()> {
    let matches = App::new("MultiLevel Feedback Queue")
                        .author("Peter Wright <peterwrighten@gmail.com>")
                        .about("A simulator to understand MLFQ")
                        .arg(Arg::with_name("seed")
                            .short("s")
                            .help("the random seed")
                            .index(1))
                        .get_matches();
}
