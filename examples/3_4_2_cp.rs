use std::io::Error;
use std::{
    env,
    fs::File,
    io::{self, copy},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("expected given 2 files");
    }
    let mut src_file = File::open(&args[1])?;
    let mut desc_file = File::create(&args[2])?; // なかったら新規作成。あればそのまま上書き

    copy(&mut src_file, &mut desc_file)?;
    Ok(())
}
