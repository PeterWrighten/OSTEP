use std::io::{prelude::*, Result};
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    println!("{} contains: \n{}", display, s);
    Ok(())
}
