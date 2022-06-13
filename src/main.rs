mod data;
mod gen;
mod mesh;
mod parse;

use clap::Parser;
use std::num::{IntErrorKind, ParseIntError};
use std::ops::Range;
use std::path::PathBuf;
use std::str::FromStr;

struct MyRange<T>(Range<T>);

impl<T> FromStr for MyRange<T>
where
    T: FromStr<Err = ParseIntError>,
{
    type Err = ParseIntError;
    fn from_str(v: &str) -> Result<MyRange<T>, Self::Err> {
        let (start, end) = v
            .split_once("..")
            .ok_or_else(|| i32::from_str_radix("a12", 10).unwrap_err())?;
        let start = start.parse()?;
        let end = end.parse()?;
        Ok(MyRange(start..end))
    }
}

#[derive(Parser)]
struct Args {
    xml: PathBuf,
    #[clap(long, default_value = "12345")]
    seed: u32,
    #[clap(long)]
    x: Option<MyRange<i32>>,
    #[clap(long)]
    y: Option<MyRange<i32>>,
    #[clap(long, default_value = "test.stl")]
    output: PathBuf,
    #[clap(short, long)]
    overrides: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let grid = parse::read_tiles(args.xml, args.overrides)?;

    gen::gen(
        &grid,
        args.output,
        args.seed,
        (args.x.map(|v| v.0), args.y.map(|v| v.0)),
    );
    Ok(())
}
