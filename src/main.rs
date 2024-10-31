use core::error;
use std::io::Read;
use std::path::Path;
use std::{env, io};

use crate::guess_img_fmt::guess_image_fmt;
use crate::parse_ext::parse_img_ext;

mod guess_img_fmt;
mod parse_ext;

type Error = Box<dyn error::Error>;

struct Args {
    output: String,
}

impl Args {
    fn parse(args: Vec<String>) -> Result<Self, Error> {
        if args.len() != 2 {
            eprintln!("Usage: {} <output_file>", args[0]);
            return Err(Box::from(String::from(
                "Invalid number of arguments: expected 1",
            )));
        }

        Ok(Args {
            output: String::from(&args[1]),
        })
    }
}

fn main() -> Result<(), Error> {
    let Args { output } = Args::parse(env::args().collect::<Vec<String>>())?;

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let input_fmt = guess_image_fmt(&input)?;
    let output_fmt = parse_img_ext(&Path::new(&output))?;

    let img = image::load_from_memory_with_format(&input, input_fmt)?;

    img.save_with_format(output, output_fmt)?;

    println!("Saved");

    Ok(())
}
